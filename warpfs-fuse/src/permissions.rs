//! Permission enforcement for WarpFS FUSE mount.
//!
//! Permission rules are glob patterns mapped to mode bits. The kernel enforces
//! the returned mode on each file, giving AI agents read/write access only to
//! files that the manifest explicitly allows.

use std::path::Path;

use crate::{FuseConfig, PermissionRule};

/// Compute the permission mode for `path` given a set of rules.
///
/// Iterates rules in order; the first rule whose glob pattern matches `path`
/// wins. If no rule matches the default is `0o644` for regular files and
/// `0o755` for directories.
pub fn compute_mode(path: &Path, rules: &[PermissionRule]) -> u32 {
    // Convert path to a forward-slash string for glob matching.
    let path_str = path.to_string_lossy();

    for rule in rules {
        for pattern in &rule.paths {
            if let Ok(glob) = glob::Pattern::new(pattern) {
                if glob.matches(&path_str) || glob.matches(path_str.trim_start_matches("./")) {
                    return rule.mode;
                }
            }
        }
    }

    // Default: 0o755 for directories, 0o644 for files.
    if path.is_dir() {
        0o755
    } else {
        0o644
    }
}

/// The default permission protections from the WarpFS spec.
///
/// These rules make infrastructure files (`.vfs/`, `.git/`, generated files,
/// lock files, vendored code) read-only at the kernel level, while allowing
/// normal read-write access to source directories.
pub fn default_protections() -> Vec<PermissionRule> {
    vec![
        // Infrastructure — strictly read-only.
        PermissionRule {
            paths: vec![".vfs/**".into()],
            mode: 0o444,
            allow_delete: false,
        },
        PermissionRule {
            paths: vec![".git/**".into()],
            mode: 0o444,
            allow_delete: false,
        },
        PermissionRule {
            paths: vec![".gitignore".into()],
            mode: 0o444,
            allow_delete: false,
        },
        PermissionRule {
            paths: vec!["**/*.sum".into()],
            mode: 0o444,
            allow_delete: false,
        },
        PermissionRule {
            paths: vec!["**/*.lock".into()],
            mode: 0o444,
            allow_delete: false,
        },
        PermissionRule {
            paths: vec!["**/*.pb.go".into()],
            mode: 0o444,
            allow_delete: false,
        },
        PermissionRule {
            paths: vec!["**/*.gen.go".into()],
            mode: 0o444,
            allow_delete: false,
        },
        PermissionRule {
            paths: vec!["**/vendor/**".into()],
            mode: 0o444,
            allow_delete: false,
        },
        PermissionRule {
            paths: vec!["**/node_modules/**".into()],
            mode: 0o444,
            allow_delete: false,
        },
        PermissionRule {
            paths: vec![".github/workflows/**".into()],
            mode: 0o444,
            allow_delete: false,
        },
        // Source directories — normal read-write.
        PermissionRule {
            paths: vec!["src/**".into()],
            mode: 0o644,
            allow_delete: true,
        },
        PermissionRule {
            paths: vec!["lib/**".into()],
            mode: 0o644,
            allow_delete: true,
        },
        PermissionRule {
            paths: vec!["cmd/**".into()],
            mode: 0o644,
            allow_delete: true,
        },
    ]
}

/// Apply permission rules to a FuseConfig (stub for future integration).
#[allow(dead_code)]
pub fn apply_rules(_config: &mut FuseConfig, _rules: &[PermissionRule]) {
    // Permission rules are applied per-file at inode creation time.
    // This hook exists for future use (e.g., loading rules from manifest).
}
