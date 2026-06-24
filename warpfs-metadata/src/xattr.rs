//! Extended attribute (xattr) read/write for WarpFS.
//!
//! All WarpFS xattrs live under the `user.vfs.*` namespace so they are
//! visible to standard tools like `getfattr -n user.vfs.relations file`.
//!
//! The `xattr` crate functions take the *full* attribute name (including the
//! `user.` prefix), so we build the full name with `format!("user.vfs.{}", name)`.

use std::path::Path;

use crate::MetadataError;

/// Build the full attribute name: `user.vfs.<name>`.
///
/// Idempotent — if `name` already has a `user.vfs.` prefix, it is
/// stripped first. This prevents double- or triple-prefixing when the
/// caller passes a fully-qualified name.
fn full_name(name: &str) -> String {
    let stripped = name.strip_prefix("user.vfs.").unwrap_or(name);
    format!("user.vfs.{}", stripped)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_name_no_prefix() {
        assert_eq!(full_name("feature"), "user.vfs.feature");
    }

    #[test]
    fn full_name_with_prefix_is_idempotent() {
        assert_eq!(full_name("user.vfs.feature"), "user.vfs.feature");
    }

    #[test]
    fn full_name_empty_name() {
        assert_eq!(full_name(""), "user.vfs.");
    }

    #[test]
    fn full_name_nested_prefix() {
        // Only strips ONE level — "user.vfs.user.vfs.foo" → "user.vfs.user.vfs.foo"
        // This is correct: the stripped part is "user.vfs." leaving "user.vfs.foo".
        assert_eq!(full_name("user.vfs.user.vfs.foo"), "user.vfs.user.vfs.foo");
    }
}

/// Set `user.vfs.<name>` on the file at `path` to `value`.
pub fn set_vfs_xattr(path: &Path, name: &str, value: &str) -> Result<(), MetadataError> {
    let attr = full_name(name);
    xattr::set(path, &attr, value.as_bytes())
        .map_err(|e| MetadataError::Xattr(e.to_string()))
}

/// Get `user.vfs.<name>` from the file at `path`.
///
/// Returns `Ok(None)` when the attribute does not exist (either the xattr
/// crate reports `None`, or the underlying syscall returns `ENODATA`).
pub fn get_vfs_xattr(path: &Path, name: &str) -> Result<Option<String>, MetadataError> {
    let attr = full_name(name);
    match xattr::get(path, &attr) {
        Ok(Some(bytes)) => Ok(Some(String::from_utf8(bytes)?)),
        Ok(None) => Ok(None),
        Err(e) => {
            // ENODATA / ENOATTR — attribute simply not set yet.
            if e.raw_os_error() == Some(libc_enodata()) {
                Ok(None)
            } else {
                Err(MetadataError::Xattr(e.to_string()))
            }
        }
    }
}

/// List all `user.vfs.*` xattrs on the file at `path`.
///
/// Returns the full attribute names (including the `user.vfs.` prefix).
pub fn list_vfs_xattrs(path: &Path) -> Result<Vec<String>, MetadataError> {
    let prefix = "user.vfs.";
    let mut result = Vec::new();
    for entry in xattr::list(path).map_err(|e| MetadataError::Xattr(e.to_string()))? {
        let name = entry.to_string_lossy().into_owned();
        if name.starts_with(prefix) {
            result.push(name);
        }
    }
    Ok(result)
}

/// Remove `user.vfs.<name>` from the file at `path`.
pub fn remove_vfs_xattr(path: &Path, name: &str) -> Result<(), MetadataError> {
    let attr = full_name(name);
    xattr::remove(path, &attr).map_err(|e| MetadataError::Xattr(e.to_string()))
}

/// Return the platform-specific errno value for "no data / attribute not found".
///
/// On Linux this is `ENODATA` (61). On macOS the xattr crate maps missing
/// attributes to `None` directly, so the value here is irrelevant.
#[cfg(target_os = "linux")]
fn libc_enodata() -> i32 {
    61 // ENODATA
}

#[cfg(not(target_os = "linux"))]
fn libc_enodata() -> i32 {
    -1 // sentinel — won't match any real errno
}
