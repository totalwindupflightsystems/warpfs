//! Permission enforcement for WarpFS FUSE mount (re-exports from warpfs_permissions).
//!
//! All permission logic now lives in the standalone `warpfs_permissions` crate.
//! This module re-exports the public API for backward compatibility.

pub use warpfs_permissions::{
    PermissionEngine, PermissionError, PermissionOp, PermissionResult,
    PermissionRule,
};

use std::path::Path;

/// Compute the permission mode for `path` given a set of rules.
///
/// Iterates rules in order; the first rule whose glob pattern matches `path`
/// wins. If no rule matches the default is `0o644`.
pub fn compute_mode(path: &Path, rules: &[PermissionRule]) -> u32 {
    let engine = PermissionEngine::from_rules(rules.to_vec());
    engine.compute_mode(path)
}

/// The default permission protections from the WarpFS spec.
pub fn default_protections() -> Vec<PermissionRule> {
    warpfs_permissions::default_protections()
}
