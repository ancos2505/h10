use h10::http::{
    headers::{ContentType, Date, Pragma, Server},
    result::H10LibResult,
    status_code::StatusCode,
};

use crate::server::ServerResponse;

pub fn styles_css() -> H10LibResult<ServerResponse> {
    let css = include_str!("../../assets/styles.css");
    #[cfg(feature = "debug")]
    dbg!(&html);

    Ok(ServerResponse::new(StatusCode::OK)
        .header(ContentType::css())
        .header(Date::now()?)
        .header(Server::default())
        .header(Pragma::default())
        .body(css))
}
