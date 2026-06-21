// WarpFS Triggers — inotify-wired event engine
//
// Every file event fires triggers:
//   1. parse-and-diff → tree-sitter AST → edge updates
//   2. User-defined commands with {{ .FilePath }} templating
//   3. upload-to-backend → S3 sync
//
// Features: debouncing, async execution, timeouts, concurrency limiting

pub mod engine;
pub mod debounce;

use std::path::PathBuf;

/// Trigger configuration from manifest.
pub struct TriggerConfig {
    pub name: String,
    pub watch_pattern: String,
    pub events: Vec<String>,       // write, delete
    pub command: Option<String>,   // shell command template
    pub builtin: Option<String>,   // parse-and-diff, upload-to-backend
    pub async_exec: bool,
    pub timeout_secs: u64,
    pub debounce_ms: u64,
    pub on_success: Option<TriggerAction>,
    pub on_failure: Option<TriggerAction>,
}

pub enum TriggerAction {
    SetXattr { key: String, value_template: String },
    Warn,
    Error,
}

/// A file event received from inotify.
pub struct FileEvent {
    pub path: PathBuf,
    pub event_type: EventType,
    pub timestamp: u64,  // unix timestamp
}

pub enum EventType {
    Write,
    Delete,
    Create,
}
