use std::{
    collections::BTreeMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread::{self, sleep},
    time::{Duration, Instant},
};

use crate::{
    http::{
        pages::Endpoint,
        proto::{response::Response, status_code::StatusCode},
    },
    result::H10ServerError,
    ServerResult, MAX_ACTIVE_SESSIONS,
};

const CHUNK_SIZE: usize = 4096;

pub struct HttpServer;
impl HttpServer {
    pub fn run() -> ServerResult<()> {
        let mut active_sessions = Arc::new(Mutex::new(0));

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
        let mut incomming = 0;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    incomming += 1;
                    dbg!(incomming);
                    let stats_mutex = Arc::clone(&prev_stats);
                    let act_session = Arc::clone(&active_sessions);
                    thread::spawn(move || {
                        Self::handle_client(&stats_mutex, stream, &act_session);
                    });
                    // incomming += 1;
                    // dbg!(incomming);
                    // let stats_mutex = Arc::clone(&prev_stats);
                    // let act_session = Arc::clone(&active_sessions);
                    // thread::spawn(move || {
                    //     if let Ok(mut data) = stats_mutex.lock() {
                    //         let _unused = Self::handle_client(&mut data, stream, act_session);
                    //     }
                    // });
                }
                Err(e) => {
                    println!("Unable to connect: {}", e);
                }
            }
        }
        Ok(())
    }
    fn handle_client(
        arc_prev_stats: &Arc<Mutex<BTreeMap<String, (u64, u64)>>>,
        // prev_stats: &mut BTreeMap<String, (u64, u64)>,
        stream: TcpStream,
        arc_act_session: &Arc<Mutex<usize>>,
    ) -> ServerResult<()> {
        let now = Instant::now();

        let act_session = Arc::clone(arc_act_session);

        let opened_sessions = match act_session.try_lock() {
            Ok(data) => Some(*data),
            Err(error) => {
                dbg!(error);
                None
            }
        };

        let response_str = if let Some(sessions) = opened_sessions {
            if sessions < MAX_ACTIVE_SESSIONS {
                match act_session.try_lock() {
                    Ok(mut data) => {
                        dbg!(*data);
                        *data += 1;
                        dbg!(*data);
                    }
                    Err(error) => {
                        dbg!(error);
                    }
                };
                Self::handle_read(&stream)?
            } else {
                let status = StatusCode::ServiceUnavailable;
                Response::new(status).to_string()
            }
        } else {
            let status = StatusCode::ServiceUnavailable;
            Response::new(status).to_string()
        };

        Self::handle_write(arc_prev_stats, stream, response_str, arc_act_session)?;
        println!(
            "Active sessions: {:?}. Response generated in {} secs. ",
            opened_sessions,
            now.elapsed().as_secs_f64(),
        );

        Ok(())
    }
    fn handle_read(mut stream: &TcpStream) -> ServerResult<String> {
        sleep(Duration::from_millis(1234));
        let mut buf = [0u8; CHUNK_SIZE];
        match stream.read(&mut buf) {
            Ok(_) => {
                let req_str = String::from_utf8_lossy(&buf);
                // println!("{}", req_str);
                Ok(Endpoint::dispatcher(req_str).to_string())
            }
            Err(e) => {
                let error = e.to_string();
                println!("Unable to read stream: {}", error);
                Err(H10ServerError(error.to_string()))
            }
        }
    }
    fn handle_write(
        // prev_stats: &mut BTreeMap<String, (u64, u64)>,
        arc_prev_stats: &Arc<Mutex<BTreeMap<String, (u64, u64)>>>,
        mut stream: TcpStream,
        response_str: String,
        act_session: &Arc<Mutex<usize>>,
    ) -> ServerResult<()> {
        // let prev_stats = Arc::clone(arc_prev_stats);
        match act_session.try_lock() {
            Ok(mut data) => {
                dbg!(*data);
                *data -= 1;
                dbg!(*data);
            }
            Err(error) => {
                dbg!(error);
            }
        };
        match stream.write(response_str.as_bytes()) {
            Ok(bytes) => println!("Response sent: {bytes} Bytes sent."),
            Err(e) => println!("Failed sending response: {}", e),
        }
        Ok(())
    }
}
