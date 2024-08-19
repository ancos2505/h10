use std::{
    collections::BTreeMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use crate::AppResult;

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
        let response = crate::http::pages::root();
        match stream.write(response.to_string().as_bytes()) {
            Ok(bytes) => println!("Response sent: {bytes} Bytes sent."),
            Err(e) => println!("Failed sending response: {}", e),
        }
        Ok(())
    }
}
