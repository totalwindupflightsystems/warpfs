//! Remote Git backend — clone, pull, and worktree management via git2.
//!
//! Clones repos to `~/.warpfs/worktrees/<name>/` and exposes them as
//! virtual directories. Supports auto-pull on configurable intervals and
//! read-only enforcement.

use std::path::{Path, PathBuf};

use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};

/// Error type for Git backend operations.
#[derive(Debug, thiserror::Error)]
pub enum GitError {
    #[error("git operation failed: {0}")]
    Git(#[from] git2::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("repository not found at {0}")]
    NotFound(PathBuf),
    #[error("repository is read-only")]
    ReadOnly,
}

pub type GitResult<T> = Result<T, GitError>;

/// Configuration for a remote Git backend mount.
pub struct GitBackendConfig {
    pub url: String,
    pub ref_name: String,
    pub at: String,
    pub writable: bool,
    pub auto_pull_secs: Option<u64>,
    pub cache_dir: Option<PathBuf>,
}

/// Manages a remote Git repository clone and its worktree.
pub struct GitBackend {
    config: GitBackendConfig,
    worktree: PathBuf,
    repo: Repository,
}

impl GitBackend {
    /// Clone (or open existing) a remote Git repository.
    ///
    /// If the worktree already exists and contains a valid repo, opens it
    /// instead of re-cloning. Auto-pulls if `auto_pull_secs` is set and
    /// the clone is older than the configured interval.
    pub fn mount(config: GitBackendConfig) -> GitResult<Self> {
        let worktree = config
            .cache_dir
            .clone()
            .unwrap_or_else(|| PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "/tmp".into())).join(".warpfs/worktrees"))
            .join(sanitize_name(&config.url));

        let repo = if worktree.join(".git").exists() {
            let repo = Repository::open(&worktree)?;

            // Auto-pull if configured and stale.
            if let Some(interval) = config.auto_pull_secs {
                if should_pull(&worktree, interval) {
                    pull_remote(&repo, &config.ref_name)?;
                }
            }

            repo
        } else {
            std::fs::create_dir_all(&worktree)?;
            let repo = Repository::clone(&config.url, &worktree)?;

            // Checkout the requested ref if not default.
            if config.ref_name != "main" && config.ref_name != "master" {
                checkout_ref(&repo, &config.ref_name)?;
            }

            repo
        };

        Ok(GitBackend { config, worktree, repo })
    }

    /// Resolve a virtual path within this backend to the real filesystem path.
    pub fn resolve(&self, virtual_path: &str) -> GitResult<PathBuf> {
        let rel = virtual_path.strip_prefix(&self.config.at).unwrap_or(virtual_path);
        let real = self.worktree.join(rel.trim_start_matches('/'));
        if !real.exists() {
            return Err(GitError::NotFound(real));
        }
        Ok(real)
    }

    /// Get backend metadata for reporting.
    pub fn info(&self) -> crate::BackendInfo {
        crate::BackendInfo {
            backend: "git".to_string(),
            real_path: self.worktree.to_string_lossy().to_string(),
            cached: true,
            cache_path: Some(self.worktree.to_string_lossy().to_string()),
            sync_status: "synced".to_string(),
        }
    }

    /// Whether writes are allowed on this backend.
    pub fn writable(&self) -> bool {
        self.config.writable
    }

    /// The virtual mount path for this backend.
    pub fn mount_point(&self) -> &str {
        &self.config.at
    }
}

// ── Helpers ────────────────────────────────────────────────────────

/// Sanitize a repo URL into a safe directory name.
fn sanitize_name(url: &str) -> String {
    url.replace("https://", "")
        .replace("http://", "")
        .replace("git@", "")
        .replace(':', "_")
        .replace('/', "_")
        .replace('.', "_")
}

/// Check if the repo's last fetch is older than `interval_secs`.
fn should_pull(worktree: &Path, interval_secs: u64) -> bool {
    let fetch_head = worktree.join(".git").join("FETCH_HEAD");
    match std::fs::metadata(&fetch_head) {
        Ok(meta) => {
            let age = meta
                .modified()
                .ok()
                .and_then(|t| t.elapsed().ok())
                .map(|d| d.as_secs())
                .unwrap_or(u64::MAX);
            age > interval_secs
        }
        Err(_) => true, // No FETCH_HEAD — needs pull.
    }
}

/// Pull the given ref from origin.
fn pull_remote(repo: &Repository, ref_name: &str) -> GitResult<()> {
    let mut remote = repo.find_remote("origin")?;

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _username, _allowed| {
        // Use SSH agent or default credentials.
        Cred::default()
    });

    let mut fetch_opts = FetchOptions::new();
    fetch_opts.remote_callbacks(callbacks);

    let refspec = format!("+refs/heads/{ref_name}:refs/remotes/origin/{ref_name}");
    remote.fetch(&[&refspec], Some(&mut fetch_opts), None)?;

    Ok(())
}

/// Checkout a specific ref (branch or tag).
fn checkout_ref(repo: &Repository, ref_name: &str) -> GitResult<()> {
    let (object, reference) = repo.revparse_ext(ref_name)?;
    repo.checkout_tree(&object, None)?;

    match reference {
        Some(gref) if gref.is_tag() => {
            repo.set_head_detached(object.id())?;
        }
        _ => {
            repo.set_head(&format!("refs/heads/{ref_name}"))?;
        }
    }

    Ok(())
}
