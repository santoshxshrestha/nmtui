use std::sync::Arc;
use std::sync::atomic::AtomicBool;

#[derive(Debug, Default)]
pub struct Flags {
    pub is_hidden: bool,
    pub is_scanning: Arc<AtomicBool>,
    pub show_delete_confirmation: bool,
    pub show_help: bool,
    pub show_password_popup: bool,
    pub show_saved: bool,
    pub show_ssid_popup: bool,
    pub show_status_popup: bool,
}
