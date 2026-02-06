use padauk::{children, column, outlined_card, scaffold, text, Widget};

pub fn example_screen(
    app_bar: impl Widget + 'static,
    content: impl Widget + 'static,
    code: &'static str,
) -> padauk::UiNode {
    let preview_block = outlined_card(children![content]).padding(8.0);
    let code_block = outlined_card(children![text(code).padding(8.0)]).padding(8.0);

    scaffold(column(children![
        text("Preview").padding(8.0),
        preview_block,
        text("Code").padding(8.0),
        code_block,
    ]))
    .app_bar(app_bar)
    .build()
}

pub fn example_screen_with_fab(
    app_bar: impl Widget + 'static,
    content: impl Widget + 'static,
    fab: impl Widget + 'static,
    code: &'static str,
) -> padauk::UiNode {
    let preview_block = outlined_card(children![content]).padding(8.0);
    let code_block = outlined_card(children![text(code).padding(8.0)]).padding(8.0);

    scaffold(column(children![
        text("Preview").padding(8.0),
        preview_block,
        text("Code").padding(8.0),
        code_block,
    ]))
    .app_bar(app_bar)
    .fab(fab)
    .build()
}
