use html_rs::{
    elements::{Div, ElementBuilder, TextContent, P},
    Html, HtmlBody, HtmlHeadItem, HtmlScript, HtmlStyle,
};

fn root<'a>() -> Html<'a> {
    let title = HtmlHeadItem::new("<title>It works!</title>");
    let style = HtmlStyle::new("body { color: #000000; }");
    let script1 = HtmlScript::new(
        format!(
            r#"console.log("Hello from file {} at line {}")"#,
            file!(),
            line!(),
        )
        .as_str(),
    );

    let div = Div::builder().attr("class", "light-theme").append_child(
        P::builder()
            .attr("class", "light-theme")
            .append_child(TextContent::text("It Works!")),
    );

    let body = HtmlBody::new().script(script1).append_child(div);

    let script2 = HtmlScript::new(
        format!(
            r#"console.log("Hello from file {} at line {}")"#,
            file!(),
            line!(),
        )
        .as_str(),
    );
    let html = Html::new()
        .head(title)
        .add_style(style)
        .add_script(script2)
        .body(body);

    #[cfg(feature = "debug")]
    dbg!(&html);

    html
}
