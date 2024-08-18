use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::AppResult;

use super::proto::headers::ContentType;
use super::proto::response::Response;
use super::proto::status_code::{StatusCode, OK};

const CHUNK_SIZE: usize = 4096;

pub struct HttpServer;
impl HttpServer {
    pub fn run() -> AppResult<()> {
        let port = 8080;
        let listener = TcpListener::bind(format!("0.0.0.0:{port}"))?;

        println!("Listening for connections on port {port}");
        let prev_stats: Arc<Mutex<BTreeMap<String, (u64, u64)>>> =
            Arc::new(Mutex::new(BTreeMap::new()));

        {
            // let stats_mutex = Arc::clone(&prev_stats);
            // let res = stats_mutex.lock();
            // if let Ok(mut data) = res {
            //     Stats::new().render(&mut data)?;
            // }
        }

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let stats_mutex = Arc::clone(&prev_stats);
                    thread::spawn(move || {
                        if let Ok(mut data) = stats_mutex.lock() {
                            let _unused = Self::handle_client(&mut data, stream);
                        }
                    });
                }
                Err(e) => {
                    println!("Unable to connect: {}", e);
                }
            }
        }
        Ok(())
    }
    fn handle_client(
        prev_stats: &mut BTreeMap<String, (u64, u64)>,
        stream: TcpStream,
    ) -> AppResult<()> {
        Self::handle_read(&stream);

        Self::handle_write(prev_stats, stream)?;
        Ok(())
    }
    fn handle_read(mut stream: &TcpStream) {
        let mut buf = [0u8; CHUNK_SIZE];
        match stream.read(&mut buf) {
            Ok(_) => {
                let req_str = String::from_utf8_lossy(&buf);
                println!("{}", req_str);
            }
            Err(e) => println!("Unable to read stream: {}", e),
        }
    }
    fn handle_write(
        prev_stats: &mut BTreeMap<String, (u64, u64)>,
        mut stream: TcpStream,
    ) -> AppResult<()> {
        // let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
        // let http_response_header =
        //     "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n";
        let status = StatusCode::<OK>::new();
        let response = Box::new(Response::new(status));

        let content_type = ContentType::html();

        let html_page_content = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="refresh" content="3">
    <title>{}/{}</title>
</head>
<body>"#,
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
        );
        let http_response_footer = "\n</body>\n</html>\r\n";

        // let terminal_output = Terminal::new().render()
        // .replace("\n", "<br/>")
        // ;
        // println!("");
        // dbg!(&terminal_output);
        // let stats = Stats::new().render(prev_stats)?;
        // let mut response = "".to_string();
        // response.push_str(http_response_header);
        // response.push_str(&html_page_content);
        // response.push_str(&stats);
        // response.push_str(&terminal_output);
        // response.push_str(&http_response_footer);
        dbg!(&response);

        let response_str = response.to_string();

        dbg!(&response_str);

        match stream.write(response_str.as_bytes()) {
            Ok(bytes) => println!("Response sent: {bytes} Bytes sent."),
            Err(e) => println!("Failed sending response: {}", e),
        }
        Ok(())
    }
}
