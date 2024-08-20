use html_rs::{
    elements::{ElementBuilder, Meta, TextContent, Title, H1},
    Html, HtmlBody,
};

use crate::{
    http::proto::{
        headers::{ContentType, Date, Pragma, Server},
        status_code::StatusCode,
    },
    result::ServerResult,
};

use crate::http::proto::response::Response;

pub fn root() -> ServerResult<Response> {
    let html = Html::builder()
        .head_item(Title::builder().append_child(TextContent::text("It works!")))
        .head_item(Meta::builder().attr("charset", "utf-8"))
        .body(
            HtmlBody::builder()
                .set_attr("lang", "en")
                .set_attr("server-name", env!("CARGO_PKG_NAME"))
                .set_attr("server-version", env!("CARGO_PKG_VERSION"))
                .append_child(H1::builder().append_child(TextContent::text("It works!"))),
        );

    #[cfg(feature = "debug")]
    dbg!(&html);

    Ok(Response::new(StatusCode::OK)
        .header(ContentType::html())
        .header(Date::now()?)
        .header(Server::default())
        .header(Pragma::default())
        .body(html))
}
