// WarpFS FUSE — kernel-level virtual filesystem
//
// Mounts repos + backends as a unified directory tree.
// The Linux kernel enforces permissions (mode bits).
// Agents use cat/ls/getfattr — no special tools needed.

pub mod daemon;
pub mod permissions;
pub mod ops;
pub mod triggers;

use std::path::PathBuf;

/// FUSE mount configuration from manifest.
pub struct FuseConfig {
    pub mount_point: PathBuf,
    pub allow_other: bool,
    pub direct_io: bool,
    pub auto_unmount: bool,
    pub attr_timeout: f64,
    pub entry_timeout: f64,
    pub max_read: u32,
    pub max_write: u32,
}

/// Permission mode enforced by the kernel.
pub struct PermissionRule {
    pub paths: Vec<String>,  // glob patterns
    pub mode: u32,           // octal (0444, 0644)
    pub allow_delete: bool,
}
