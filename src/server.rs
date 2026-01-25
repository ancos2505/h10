mod cli;
mod counter;
mod helper;
mod result;
mod traits;
use std::{
    collections::BTreeMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread::{self},
    time::Instant,
};

use h10::{error, log::LogLevel};

use crate::{
    CLI_ARGS, HTTP10_STRICT_MODE, MAX_ACTIVE_SESSIONS, TEN_KBYTES, pages::Endpoint,
    server::helper::now_in_unix_epoch,
};

use h10::{
    debug,
    http::{
        headers::{Connection, IntoHeader},
        response::Response,
        result::H10LibResult,
        status_code::StatusCode,
    },
    trace,
};

pub(crate) use self::cli::{CliHttp10StrictMode, CliVerboseMode};

pub(crate) use self::{
    cli::Cli,
    result::{ServerError, ServerResult},
    traits::IntoResponse,
};

pub(crate) struct ServerResponse(Response);

impl ServerResponse {
    pub fn new(status: StatusCode) -> Self {
        if HTTP10_STRICT_MODE.get().is_some() {
            Self(Response::new(status))
        } else {
            Self(Response::new(status).add_header(Connection::default()))
        }
    }
    pub fn add_header<H: IntoHeader>(self, header: H) -> Self {
        Self(self.0.add_header(header))
    }
    pub fn set_body<B: AsRef<str>>(self, body: B) -> Self {
        Self(self.0.set_body(body))
    }
}

impl IntoResponse for ServerResponse {
    fn into_response(self) -> Response {
        self.0
    }
}

pub(crate) struct HttpServer;
impl HttpServer {
    /// # MIN_REQ_SIZE_BYTES
    /// Minimal request (To be defined) METHOD,HEADER,BODY,TERM
    /// ```
    /// GET / (5 Bytes)
    /// \r\n\r\n (4 Bytes)
    /// 5 + 4 = 10 Bytes
    /// ```
    const MIN_REQ_SIZE_BYTES: usize = 10;
    const MAX_REQ_SIZE_BYTES: usize = Self::CHUNK_SIZE * 1024; // 10 MBytes

    // const CHUNK_SIZE: usize = TEN_KBYTES;
    const CHUNK_SIZE: usize = Self::MIN_REQ_SIZE_BYTES;

    fn listener(cli_data: &Cli) -> String {
        format!("{}:{}", cli_data.ip_address, cli_data.port)
    }
    pub fn run() -> ServerResult<()> {
        if let Some(cli) = CLI_ARGS.get() {
            if cli.is_help {
                Cli::usage();
                return Ok(());
            }

            let mut active_sessions = Arc::new(Mutex::new(0));

            let list_str = Self::listener(cli);
            let listener = TcpListener::bind(&list_str)?;
            // let listener = TcpListener::bind(&list_str)?;

            println!("Listening for connections on {}", list_str);
            let prev_stats: Arc<Mutex<BTreeMap<String, (u64, u64)>>> =
                Arc::new(Mutex::new(BTreeMap::new()));

            {
                // let stats_mutex = Arc::clone(&prev_stats);
                // let res = stats_mutex.lock();
                // if let Ok(mut data) = res {
                //     Stats::new().render(&mut data)?;
                // }
            }

            let mut incomming: usize = 0;

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        incomming += 1;

                        // dbg!(incomming);
                        let stats_mutex = Arc::clone(&prev_stats);
                        // TODO: It seems that Mutex it's being poisoned here, because of that active sessions are not being counted properly.
                        let act_session = Arc::clone(&active_sessions);
                        // TODO: Implement active sessions manager (What is a session? Why it is hangs?)

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

        let opened_sessions = count_active_sessions(&act_session)?;

        dbg!(opened_sessions);

        let response_str = if let Some(sessions) = opened_sessions {
            if sessions < MAX_ACTIVE_SESSIONS {
                open_active_session(&act_session)?;
                let server_response = match Self::handle_read(&stream) {
                    Ok(res) => res,
                    Err(error) => {
                        let status = StatusCode::ServiceUnavailable;
                        eprintln!(
                            "Error(status): {error} on file [{}] in line {}",
                            file!(),
                            line!()
                        );
                        ServerResponse::new(status)
                    }
                };
                server_response
            } else {
                let status = StatusCode::ServiceUnavailable;
                eprintln!(
                    "Error(status): MAX_ACTIVE_SESSIONS reached! on file [{}] in line {}",
                    file!(),
                    line!()
                );
                ServerResponse::new(status)
            }
        } else {
            let status = StatusCode::ServiceUnavailable;
            eprintln!(
                "Error(status): `opened_sessions` is `None` on file [{}] in line {}",
                file!(),
                line!()
            );
            ServerResponse::new(status)
        };
        close_active_session(&act_session)?;
        match Self::handle_write(arc_prev_stats, stream, response_str, arc_act_session) {
            Ok(_) => {
                println!(
                    "Response generated in {} secs. ",
                    now.elapsed().as_secs_f64(),
                );
            }
            Err(err) => {
                println!("Error on sending Reponse. Reason: {err}.",);
            }
        }

        let opened_sessions = count_active_sessions(&act_session)?;
        println!(
            "Active sessions: {:?}. Response generated in {} secs. ",
            opened_sessions,
            now.elapsed().as_secs_f64(),
        );
        Ok(())
    }

    fn handle_read(mut stream: &TcpStream) -> ServerResult<ServerResponse> {
        // TODO: Probably improve CHUNK_SIZE implementation.
        // TODO:  Implement Request parser properly in order to handle requests
        // TODO: larger than CHUNK_SIZE.

        let mut request_bytes: Vec<u8> = vec![];
        // TODO: Implement Error handling properly.
        let mut buf: [u8; Self::CHUNK_SIZE] = [0u8; Self::CHUNK_SIZE];

        let mut stream_reading = true;
        let mut chunk_counter: usize = 0;
        while stream_reading {
            chunk_counter += 1;

            debug!("Read bytes to chunck #{chunk_counter}");

            // TODO: stream.read() with timeout.
            // TODO: Prevent slowloris attack.
            // TODO: Improve error handling.
            let bytes_read = stream.read(&mut buf)?;
            dbg!(bytes_read);
            if bytes_read > 0 {
                for idx in 0..bytes_read {
                    request_bytes.push(buf[idx])
                }

                let len = request_bytes.len();
                // TODO:  Check the right approach based in the HTTP RFC in
                // TODO: | FIRST_LIST | \r\n\r\n
                // TODO: | HEADERS    | \r\n\r\n
                // TODO: | BODY       | \r\n\r\n (Optional depends on method) - check RFC
                if len > Self::MIN_REQ_SIZE_BYTES
                    && request_bytes[len - 4] == b'\r'
                    && request_bytes[len - 3] == b'\n'
                    && request_bytes[len - 2] == b'\r'
                    && request_bytes[len - 1] == b'\n'
                // '\n'
                {
                    stream_reading = false;
                }
            } else {
                stream_reading = false;
            };
        }

        let request_lenth_in_bytes = request_bytes.len();

        #[cfg(debug_assertions)]
        {
            trace!("Request bytes: {request_bytes:?}");
            let end_idx = request_lenth_in_bytes - 1;
            let start_idx = end_idx - 4;
            dbg!(start_idx, end_idx);
            for idx in start_idx..=end_idx {
                dbg!(request_bytes[idx]);
            }
            debug!("Request length in bytes: {request_lenth_in_bytes}");

            let last_chunk = request_bytes
                .get((request_lenth_in_bytes - Self::MIN_REQ_SIZE_BYTES)..(request_lenth_in_bytes))
                .map(String::from_utf8_lossy);

            dbg!(last_chunk);
        }

        debug!("Request received: {request_lenth_in_bytes} Bytes.");

        // TODO: Implement Request Handling properly.
        Ok(Endpoint::dispatcher(&request_bytes))
    }

    fn handle_write(
        // prev_stats: &mut BTreeMap<String, (u64, u64)>,
        arc_prev_stats: &Arc<Mutex<BTreeMap<String, (u64, u64)>>>,
        mut stream: TcpStream,
        server_response: ServerResponse,
        act_session: &Arc<Mutex<usize>>,
    ) -> H10LibResult<()> {
        // TODO: Implement Request generator properly.
        // let prev_stats = Arc::clone(arc_prev_stats);
        // CURRENT_PENDING:

        let statuscode_str = server_response.0.status().to_string();
        let response_str = server_response.into_response().to_string();
        match stream.write(response_str.as_bytes()) {
            Ok(bytes) => {
                debug!(format!("Response status code: {statuscode_str}"));
                debug!(format!("Response sent: {bytes} Bytes."));
                trace!(format!("{response_str}"));
            }
            Err(err) => error!(format!("Failed sending response: {err}")),
        }
        Ok(())
    }
}

fn count_active_sessions(act_session: &Arc<Mutex<usize>>) -> ServerResult<Option<usize>> {
    let mut errors_count = 0;
    for _ in 1..=10 {
        match act_session.try_lock() {
            Ok(data) => {
                return Ok(Some(*data));
            }
            Err(err) => {
                dbg!(err);
                errors_count += 1;
            }
        }
    }
    if errors_count > 0 {
        dbg!(errors_count);
    }
    Ok(None)
}

fn open_active_session(act_session: &Arc<Mutex<usize>>) -> ServerResult<()> {
    let mut errors_count = 0;
    for _ in 1..=10 {
        match act_session.try_lock() {
            Ok(mut data) => {
                if *data <= MAX_ACTIVE_SESSIONS {
                    *data += 1;
                    break;
                } else {
                    continue;
                }
            }
            Err(err) => {
                dbg!(err);
                errors_count += 1;
            }
        }
    }
    if errors_count > 0 {
        dbg!(errors_count);
    }
    Ok(())
}

fn close_active_session(act_session: &Arc<Mutex<usize>>) -> ServerResult<()> {
    let mut errors_count = 0;
    for _ in 1..=10 {
        match act_session.try_lock() {
            Ok(mut data) => {
                if *data > 0 {
                    *data -= 1;
                    break;
                }
            }
            Err(err) => {
                dbg!(err);
                errors_count += 1;
            }
        }
    }
    if errors_count > 0 {
        dbg!(errors_count);
    }
    Ok(())
}
