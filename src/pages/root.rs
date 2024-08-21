use html_rs::{
    elements::{Button, Div, ElementBuilder, Link, Meta, TextContent, Title, H1},
    Html, HtmlBody,
};

use h10::http::{
    headers::{ContentType, Date, Pragma, Server},
    result::H10LibResult,
    status_code::StatusCode,
};

use crate::server::ServerResponse;

pub fn root() -> H10LibResult<ServerResponse> {
    let card = Div::builder().attr("class", "card").append_child(
        Button::builder()
            .attr("id", "counter")
            .attr("type", "button")
            // TODO
            .append_child(TextContent::text("count is 0")),
    );

    let div = Div::builder().append_child(
        Div::builder()
            .append_child(H1::builder().append_child(TextContent::text("It works!")))
            .append_child(card),
    );
    let html = Html::builder()
        .head_item(Title::builder().append_child(TextContent::text(format!("{} v{}",env!("CARGO_PKG_NAME"),env!("CARGO_PKG_VERSION")))))
        .head_item(Meta::builder().attr("charset", "utf-8"))
        .head_item(
            Meta::builder()
                .attr("name", "viewport")
                .attr("content", "width=device-width, initial-scale=1.0"),
        )
        .head_item(
            Link::builder()
                .attr("href", "/assets/styles.css")
                .attr("rel", "stylesheet")
                .attr("type", "text/css"),
        )
        .body(
            HtmlBody::builder()
                .set_attr("lang", "en")
                .set_attr("server-name", env!("CARGO_PKG_NAME"))
                .set_attr("server-version", env!("CARGO_PKG_VERSION"))
                .append_child(Div::builder().attr("id", "app").append_child(div)),
        );

    #[cfg(feature = "debug")]
    dbg!(&html);

    Ok(ServerResponse::new(StatusCode::OK)
        .header(ContentType::html())
        .header(Date::now()?)
        .header(Server::default())
        .header(Pragma::default())
        .body(html))
}
