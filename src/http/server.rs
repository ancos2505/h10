use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use html_rs::elements::{ElementBuilder, TextContent, H1};
use html_rs::{Html, HtmlBody, HtmlHead, HtmlHeadItem};

use crate::http::proto::headers::Connection;
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
        let html = Html::new()
            .head(HtmlHeadItem::new(r#"<meta charset="utf-8">"#))
            .head(HtmlHeadItem::new("<title>It works!</title>"))
            .body(
                HtmlBody::new()
                    .set_attr("lang", "en")
                    .set_attr("server-name", env!("CARGO_PKG_NAME"))
                    .set_attr("server-version", env!("CARGO_PKG_VERSION"))
                    .append_child(H1::builder().append_child(TextContent::text("It works!"))),
            );

        let status = StatusCode::<OK>::new();

        let response = Response::new(status)
            .header(ContentType::html())
            .header(Connection::close())
            .body_html(html);

        match stream.write(response.to_string().as_bytes()) {
            Ok(bytes) => println!("Response sent: {bytes} Bytes sent."),
            Err(e) => println!("Failed sending response: {}", e),
        }
        Ok(())
    }
}
