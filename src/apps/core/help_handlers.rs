use super::App;

use crossterm::event::poll;
use crossterm::event::{self, Event, KeyEvent};
use std::{io, time::Duration};

impl App {
    /// Process a single input event while the help view is active.
    ///
    /// If a key event is available, handles it as follows:
    /// - Esc, Enter, or the `q` key will close the help view.
    /// - Ctrl+C will exit the application.
    ///  All other events are ignored. I/O errors from polling or reading input are propagated.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut app = App::new();
    /// // When help is shown, process a single input event (may return an I/O error).
    /// app.handle_help().unwrap();
    /// ```
    pub fn handle_help(&mut self) -> io::Result<()> {
        if poll(Duration::from_micros(1))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: event::KeyCode::Esc,
                    kind: event::KeyEventKind::Press,
                    ..
                }) => {
                    self.close_help();
                }
                Event::Key(KeyEvent {
                    code: event::KeyCode::Enter,
                    kind: event::KeyEventKind::Press,
                    ..
                }) => {
                    self.close_help();
                }
                Event::Key(KeyEvent {
                    code: event::KeyCode::Char('q'),
                    kind: event::KeyEventKind::Press,
                    ..
                }) => {
                    self.close_help();
                }
                Event::Key(KeyEvent {
                    code: event::KeyCode::Char('c'),
                    kind: event::KeyEventKind::Press,
                    modifiers: event::KeyModifiers::CONTROL,
                    ..
                }) => {
                    self.exit();
                }
                _ => {}
            };
        }
        Ok(())
    }
    /// Hide the help view.
    ///
    /// Sets the application's help-visible flag to `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut app = App::default();
    /// app.flags.show_help = true;
    /// app.close_help();
    /// assert!(!app.flags.show_help);
    /// ```
    pub fn close_help(&mut self) {
        self.flags.show_help = false;
    }
}
