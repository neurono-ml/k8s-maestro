use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum PackageSource {
    Git {
        url: String,
        branch: Option<String>,
        path: Option<String>,
    },
    RemotePath {
        url: String,
    },
    LocalPath {
        path: PathBuf,
    },
    Registry {
        registry: String,
        package_name: String,
        version: String,
    },
}

#[derive(Debug, Clone)]
pub struct PackageCache {
    cache_dir: PathBuf,
}

impl PackageCache {
    pub fn new() -> Result<Self> {
        let cache_dir = std::env::temp_dir().join("k8s-maestro").join("cache");
        fs::create_dir_all(&cache_dir).context("Failed to create cache directory")?;
        Ok(Self { cache_dir })
    }

    pub fn get_cache_path(&self, source: &PackageSource) -> PathBuf {
        let key = Self::generate_cache_key(source);
        self.cache_dir.join(key)
    }

    fn generate_cache_key(source: &PackageSource) -> String {
        let mut hasher = Sha256::new();
        match source {
            PackageSource::Git { url, branch, path } => {
                hasher.update(url.as_bytes());
                if let Some(b) = branch {
                    hasher.update(b.as_bytes());
                }
                if let Some(p) = path {
                    hasher.update(p.as_bytes());
                }
            }
            PackageSource::RemotePath { url } => {
                hasher.update(url.as_bytes());
            }
            PackageSource::LocalPath { path } => {
                if let Ok(canon) = fs::canonicalize(path) {
                    hasher.update(canon.to_string_lossy().as_bytes());
                } else {
                    hasher.update(path.to_string_lossy().as_bytes());
                }
            }
            PackageSource::Registry {
                registry,
                package_name,
                version,
            } => {
                hasher.update(registry.as_bytes());
                hasher.update(package_name.as_bytes());
                hasher.update(version.as_bytes());
            }
        }
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Debug)]
pub struct PackageLoader {
    cache: PackageCache,
}

impl PackageLoader {
    pub fn new() -> Result<Self> {
        Ok(Self {
            cache: PackageCache::new()?,
        })
    }

    pub fn load(&self, source: &PackageSource) -> Result<PathBuf> {
        let cache_path = self.cache.get_cache_path(source);

        if cache_path.exists() {
            return Ok(cache_path);
        }

        match source {
            PackageSource::Git { url, branch, path } => {
                self.fetch_git(url, branch.as_deref(), path.as_deref())
            }
            PackageSource::RemotePath { url } => self.fetch_remote(url),
            PackageSource::LocalPath { path } => self.validate_local(path),
            PackageSource::Registry { .. } => {
                anyhow::bail!("Registry support not yet implemented")
            }
        }
    }

    fn fetch_git(&self, url: &str, _branch: Option<&str>, _path: Option<&str>) -> Result<PathBuf> {
        let cache_key = PackageCache::generate_cache_key(&PackageSource::Git {
            url: url.to_string(),
            branch: _branch.map(|s| s.to_string()),
            path: _path.map(|s| s.to_string()),
        });
        let clone_dir = self.cache.cache_dir.join(&cache_key);

        fs::create_dir_all(&clone_dir)?;

        let repo = git2::Repository::clone(url, &clone_dir)
            .with_context(|| format!("Failed to clone repository: {}", url))?;

        if let Some(branch) = _branch {
            let (obj, reference) = repo.revparse_ext(branch)?;
            repo.checkout_tree(&obj, None)?;
            repo.set_head(reference.unwrap().name().unwrap())?;
        }

        Ok(clone_dir)
    }

    fn fetch_remote(&self, url: &str) -> Result<PathBuf> {
        let cache_key = PackageCache::generate_cache_key(&PackageSource::RemotePath {
            url: url.to_string(),
        });
        let file_path = self.cache.cache_dir.join(&cache_key);

        if file_path.exists() {
            return Ok(file_path);
        }

        let response =
            reqwest::blocking::get(url).with_context(|| format!("Failed to fetch: {}", url))?;

        let bytes = response.bytes()?;
        fs::write(&file_path, bytes)?;

        Ok(file_path)
    }

    fn validate_local(&self, path: &Path) -> Result<PathBuf> {
        let canonical = path
            .canonicalize()
            .with_context(|| format!("Path does not exist: {:?}", path))?;

        if !canonical.exists() {
            anyhow::bail!("Path does not exist: {:?}", canonical);
        }

        Ok(canonical)
    }
}

impl Default for PackageLoader {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_source_display() {
        let git_source = PackageSource::Git {
            url: "https://github.com/test/repo".to_string(),
            branch: Some("main".to_string()),
            path: Some("packages/core".to_string()),
        };
        assert!(matches!(git_source, PackageSource::Git { .. }));

        let remote_source = PackageSource::RemotePath {
            url: "https://example.com/package.tar.gz".to_string(),
        };
        assert!(matches!(remote_source, PackageSource::RemotePath { .. }));

        let local_source = PackageSource::LocalPath {
            path: PathBuf::from("/local/packages/core"),
        };
        assert!(matches!(local_source, PackageSource::LocalPath { .. }));

        let registry_source = PackageSource::Registry {
            registry: "https://registry.io".to_string(),
            package_name: "my-package".to_string(),
            version: "1.0.0".to_string(),
        };
        assert!(matches!(registry_source, PackageSource::Registry { .. }));
    }

    #[test]
    fn test_package_cache_key_generation() {
        let source1 = PackageSource::Git {
            url: "https://github.com/test/repo".to_string(),
            branch: Some("main".to_string()),
            path: Some("packages/core".to_string()),
        };
        let key1 = PackageCache::generate_cache_key(&source1);

        let source2 = PackageSource::Git {
            url: "https://github.com/test/repo".to_string(),
            branch: Some("main".to_string()),
            path: Some("packages/core".to_string()),
        };
        let key2 = PackageCache::generate_cache_key(&source2);

        assert_eq!(key1, key2);

        let source3 = PackageSource::Git {
            url: "https://github.com/test/repo".to_string(),
            branch: Some("develop".to_string()),
            path: None,
        };
        let key3 = PackageCache::generate_cache_key(&source3);

        assert_ne!(key1, key3);
    }

    #[test]
    fn test_package_cache_get_path() {
        let cache = PackageCache::new().unwrap();

        let source = PackageSource::RemotePath {
            url: "https://example.com/package.tar.gz".to_string(),
        };
        let path = cache.get_cache_path(&source);

        assert!(path.starts_with(cache.cache_dir));
        assert!(path.to_string_lossy().contains("k8s-maestro"));
    }
}
