use html_rs::{
    elements::{ElementBuilder, Meta, TextContent, Title, H1},
    Html, HtmlBody,
};

use crate::http::proto::{
    headers::{Connection, ContentType},
    status_code::{StatusCode, OK},
};

use super::proto::response::Response;

pub fn root() -> Response<OK> {
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

    Response::new(StatusCode::<OK>::new())
        .header(ContentType::html())
        .header(Connection::close())
        .body(html)
}
