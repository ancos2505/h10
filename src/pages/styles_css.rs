use h10::http::{
    headers::{ContentType, Date, Pragma, Server},
    status_code::StatusCode,
};

use crate::server::{ServerResponse, ServerResult};

pub fn styles_css() -> ServerResult<ServerResponse> {
    let css = include_str!("../../assets/styles.css");

    Ok(ServerResponse::new(StatusCode::OK)
        .add_header(ContentType::css())
        .add_header(Date::now()?)
        .add_header(Server::default())
        .add_header(Pragma::default())
        .set_body(css))
}
