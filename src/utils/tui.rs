use crate::apps::core::App;
pub fn tui() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::try_restore().expect("Failed to restore terminal");
    app_result
}
