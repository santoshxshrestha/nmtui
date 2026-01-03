use std::sync::Arc;
use std::sync::atomic::AtomicBool;

#[derive(Debug, Default)]
pub struct Flags {
    // indicates whether the selected network is hidden or not
    // the ssid popup is handled by show_ssid_popup flag itself though this will be useful while connecting to hidden networks
    // so we can set the hidden flag accordingly
    pub is_hidden: bool,

    // indicates whether a scan operation is currently in progress
    // this is currently use to show a loading indicator in the ui while scanning is going on
    pub is_scanning: Arc<AtomicBool>,

    pub show_delete_confirmation: bool,
    pub show_help: bool,
    pub show_password_popup: bool,
    pub show_saved: bool,
    pub show_ssid_popup: bool,
    pub show_status_popup: bool,
}
