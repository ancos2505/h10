use crate::{
    constants::MAX_RESPONSE_LENGTH,
    http::{request::Request, response::Response, result::H10LibResult},
};

#[derive(Debug)]
pub struct HttpClient;
impl HttpClient {
    pub fn launch<S: AsRef<str>>(request: Request, connect_str: S) -> H10LibResult<Response> {
        use std::io::{Read, Write};
        use std::net::TcpStream;
        // use std::time::Instant;

        // let start = Instant::now();

        let mut response_buffer: [u8; MAX_RESPONSE_LENGTH] = [0; MAX_RESPONSE_LENGTH];

        let mut stream = TcpStream::connect(connect_str.as_ref())?;

        let request_str = request.to_string();

        stream.write_all(request_str.as_bytes())?;

        stream.read(&mut response_buffer)?;

        let response = Response::parse(&response_buffer)?;

        Ok(response)
    }
}
