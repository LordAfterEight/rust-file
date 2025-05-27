mod app;

fn main() {
    let mut terminal = ratatui::init();
    let app_result = app::App::default().run(&mut terminal);
    ratatui::restore();
    app_result.unwrap()
}
