// S3 read-only backend implementation
use aws_sdk_s3 as s3;
use aws_config;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs;

/// Errors specific to S3 backend operations.
#[derive(Debug, thiserror::Error)]
pub enum S3Error {
    #[error("s3: not found: {0}")]
    NotFound(String),
    #[error("s3: bucket operation failed: {0}")]
    BucketError(String),
    #[error("s3: read-only mount — writes rejected")]
    ReadOnly,
    #[error("s3: io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("s3: aws error: {0}")]
    Aws(String),
}

impl From<s3::Error> for S3Error {
    fn from(e: s3::Error) -> Self {
        S3Error::Aws(e.to_string())
    }
}

impl<E, R> From<s3::error::SdkError<E, R>> for S3Error
where
    E: std::fmt::Display,
    R: std::fmt::Debug,
{
    fn from(e: s3::error::SdkError<E, R>) -> Self {
        S3Error::Aws(e.to_string())
    }
}

/// Result type for S3 backend operations.
pub type S3Result<T> = Result<T, S3Error>;

/// Cached file metadata stored alongside the cached content.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct CacheMeta {
    pub s3_key: String,
    pub etag: Option<String>,
    pub content_type: Option<String>,
    pub content_length: i64,
    pub cached_at: u64,   // UNIX epoch seconds
}

/// S3 read-only client with local cache.
pub struct S3Client {
    client: s3::Client,
    cache_dir: PathBuf,
    ttl_seconds: u32,
}

impl S3Client {
    /// Create a new S3 client for a specific bucket/prefix.
    /// cache_dir: local directory for cached files (e.g., .vfs/cache/)
    /// ttl_seconds: cache TTL; 0 means never expire
    pub async fn new(region: &str, cache_dir: &Path, ttl_seconds: u32) -> S3Result<Self> {
        let config = aws_config::from_env()
            .region(s3::config::Region::new(region.to_string()))
            .load()
            .await;
        let client = s3::Client::new(&config);
        Ok(Self {
            client,
            cache_dir: cache_dir.to_path_buf(),
            ttl_seconds,
        })
    }

    /// Read an object from S3, caching it locally.
    /// Returns the local cache path on success.
    pub async fn get_object(&self, bucket: &str, key: &str) -> S3Result<PathBuf> {
        // 1. Check cache first
        let cache_path = self.cache_path(bucket, key);
        if self.is_cache_fresh(&cache_path) {
            return Ok(cache_path);
        }

        // 2. Fetch from S3
        let resp = self.client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| {
                if format!("{}", e).contains("NoSuchKey") {
                    S3Error::NotFound(key.to_string())
                } else {
                    S3Error::from(e)
                }
            })?;

        // 3. Collect bytes
        let body = resp.body.collect().await.map_err(|e| S3Error::Aws(e.to_string()))?;
        let bytes = body.into_bytes();

        // 4. Ensure cache directory exists
        if let Some(parent) = cache_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // 5. Write to cache
        fs::write(&cache_path, &bytes).await?;

        // 6. Write cache metadata
        let meta = CacheMeta {
            s3_key: key.to_string(),
            etag: resp.e_tag.clone(),
            content_type: resp.content_type.clone(),
            content_length: resp.content_length.unwrap_or(0),
            cached_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };
        let meta_json = serde_json::to_string(&meta).unwrap();
        let meta_path = cache_path.with_extension("json");
        fs::write(&meta_path, meta_json).await?;

        Ok(cache_path)
    }

    /// List objects in S3 under the given prefix.
    pub async fn list_objects(&self, bucket: &str, prefix: &str) -> S3Result<Vec<String>> {
        let resp = self.client
            .list_objects_v2()
            .bucket(bucket)
            .prefix(prefix)
            .send()
            .await?;

        let keys: Vec<String> = resp.contents()
            .iter()
            .filter_map(|obj| obj.key().map(|k| k.to_string()))
            .collect();

        Ok(keys)
    }

    /// Check if a cached file exists and is within TTL.
    fn is_cache_fresh(&self, cache_path: &Path) -> bool {
        if !cache_path.exists() {
            return false;
        }
        if self.ttl_seconds == 0 {
            return true; // TTL=0 means never expire
        }

        match cache_path.metadata() {
            Ok(meta) => {
                match meta.modified() {
                    Ok(mtime) => {
                        let elapsed = SystemTime::now()
                            .duration_since(mtime)
                            .unwrap_or_default();
                        elapsed.as_secs() < self.ttl_seconds as u64
                    }
                    Err(_) => false,
                }
            }
            Err(_) => false,
        }
    }

    /// Compute the local cache path for an S3 key.
    fn cache_path(&self, bucket: &str, key: &str) -> PathBuf {
        self.cache_dir
            .join(bucket)
            .join(key.trim_start_matches('/'))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use tokio::fs;

    // Test: cache path computation
    fn cache_path(bucket: &str, key: &str) -> PathBuf {
        Path::new("/tmp/test-cache").join(bucket).join(key.trim_start_matches('/'))
    }

    #[test]
    fn test_cache_path_basic() {
        let p = cache_path("my-bucket", "foo/bar.txt");
        assert_eq!(p, Path::new("/tmp/test-cache/my-bucket/foo/bar.txt"));
    }

    #[test]
    fn test_cache_path_leading_slash() {
        let p = cache_path("my-bucket", "/foo/bar.txt");
        assert_eq!(p, Path::new("/tmp/test-cache/my-bucket/foo/bar.txt"));
    }

    // Test: CacheMeta serialization round-trip
    #[test]
    fn test_cache_meta_roundtrip() {
        let meta = CacheMeta {
            s3_key: "prod/models/checkpoint.pt".into(),
            etag: Some("\"abc123\"".into()),
            content_type: Some("application/octet-stream".into()),
            content_length: 524288000,
            cached_at: 1719000000,
        };
        let json = serde_json::to_string(&meta).unwrap();
        let parsed: CacheMeta = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.s3_key, meta.s3_key);
        assert_eq!(parsed.etag, meta.etag);
        assert_eq!(parsed.content_length, meta.content_length);
    }

    // Test: S3Error formatting
    #[test]
    fn test_s3_error_display() {
        assert_eq!(
            S3Error::ReadOnly.to_string(),
            "s3: read-only mount — writes rejected"
        );
        assert_eq!(
            S3Error::NotFound("key.txt".into()).to_string(),
            "s3: not found: key.txt"
        );
        assert_eq!(
            S3Error::BucketError("boom".into()).to_string(),
            "s3: bucket operation failed: boom"
        );
    }
}
