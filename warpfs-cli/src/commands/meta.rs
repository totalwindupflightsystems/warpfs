//! `warpfs meta <path>` — list WarpFS extended attributes on a file.

use std::path::Path;

use anyhow::{Context, Result};
use warpfs_metadata::xattr;

/// Print every `user.vfs.*` extended attribute on `path` (prefix stripped).
///
/// If the file has no WarpFS metadata a short message is printed instead.
pub fn run(path: &str) -> Result<()> {
    let file_path = Path::new(path);

    if !file_path.exists() {
        anyhow::bail!("no such file: {path}");
    }

    let attrs = xattr::list_vfs_xattrs(file_path)
        .with_context(|| format!("failed to list xattrs for {path}"))?;

    if attrs.is_empty() {
        println!("No WarpFS metadata for {path}");
        return Ok(());
    }

    let prefix = "user.vfs.";
    for full_name in &attrs {
        // list_vfs_xattrs returns full names like "user.vfs.imports".
        // Strip the prefix for display, then fetch via the stripped key
        // (get_vfs_xattr re-adds the prefix internally).
        let stripped = full_name.strip_prefix(prefix).unwrap_or(full_name);
        let value = xattr::get_vfs_xattr(file_path, stripped)
            .with_context(|| format!("failed to read xattr {full_name}"))?;
        match value {
            Some(v) => println!("{stripped}: {v}"),
            None => println!("{stripped}:"),
        }
    }

    Ok(())
}
