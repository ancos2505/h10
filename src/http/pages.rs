use html_rs::{
    elements::{ElementBuilder, TextContent, H1},
    Html, HtmlBody, HtmlHeadItem,
};

use crate::http::proto::{
    headers::{Connection, ContentType},
    status_code::{StatusCode, OK},
};

use super::proto::response::Response;

pub fn root() -> Response<OK> {
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

    #[cfg(feature = "debug")]
    dbg!(&html);

    Response::new(StatusCode::<OK>::new())
        .header(ContentType::html())
        .header(Connection::close())
        .body(html)
}
