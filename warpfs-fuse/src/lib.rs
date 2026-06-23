// WarpFS FUSE — kernel-level virtual filesystem
//
// Mounts repos + backends as a unified directory tree.
// The Linux kernel enforces permissions (mode bits).
// Agents use cat/ls/getfattr — no special tools needed.

pub mod daemon;
pub mod ops;
pub mod permissions;
pub mod triggers;
pub mod workspace_mount;

use std::path::PathBuf;

pub use ops::{InodeEntry, InodeKind, WarpFS};

/// FUSE mount configuration from manifest.
#[derive(Clone)]
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
    pub paths: Vec<String>, // glob patterns
    pub mode: u32,          // octal (0444, 0644)
    pub allow_delete: bool,
}
