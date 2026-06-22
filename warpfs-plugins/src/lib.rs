// WarpFS Plugins — extism wasm runtime
//
// Plugins are .wasm modules loaded from .vfs/plugins/.
// Written in any language with an extism PDK (Rust, Go, Python, JS, C, Zig).
// Hot-loaded on manifest change. Sandboxed — no filesystem access except host functions.

pub mod runtime;
pub mod host_functions;
pub mod registry;

// Re-exports for ergonomic access.
pub use runtime::PluginRuntime;
pub use host_functions::HostFunctions;
pub use registry::{HookRef, PluginManifest, PluginRegistry};

/// A loaded plugin instance.
pub struct PluginInstance {
    pub name: String,
    pub wasm_path: std::path::PathBuf,
    pub hooks: Vec<HookConfig>,
    pub edge_types: Vec<String>,
    pub metadata_namespaces: Vec<String>,
}

pub struct HookConfig {
    pub on: String,         // file_write, file_read, etc.
    pub priority: u32,
    pub languages: Vec<String>,
}

/// Results returned by plugin hook execution.
pub enum HookResult {
    AddEdge { from: String, to: String, relation: String },
    SetXattr { path: String, key: String, value: String },
    Warning { path: String, message: String },
}
