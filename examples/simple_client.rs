use h10::{
    client::HttpClient,
    http::{request::Request, result::H10LibResult, url_path::UrlPath},
};

fn main() -> H10LibResult<()> {
    use std::time::Instant;

    let start = Instant::now();
    println!("\n\nRun the server first in other terminal: `cargo run -- --verbose`\n\n");

    let connect_str = "localhost:8080";

    let request = Request::get().path(UrlPath::root()).finish();
    println!("Request to launch:");
    println!("{request}");

    let response = HttpClient::launch(request, connect_str)?;

    println!(
        "Response from http://{connect_str} in {} secs:",
        start.elapsed().as_secs_f32()
    );
    println!("{response}");

    // dbg!(response, bytes_read, start.elapsed().as_millis());
    // assert_eq!(expected_str, &*request.to_string())
    Ok(())
}
