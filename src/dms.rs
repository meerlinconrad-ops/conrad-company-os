#![allow(dead_code)]
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub struct DocumentManager {
    base_dir: PathBuf,
}

impl DocumentManager {
    pub fn new(base_path: &str) -> Result<Self> {
        let dir = PathBuf::from(base_path);
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        Ok(Self { base_dir: dir })
    }

    pub fn save_file(&self, filename: &str, bytes: &[u8]) -> Result<PathBuf> {
        // Sanitize filename to prevent path traversal
        let clean_filename = Path::new(filename)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unnamed_document");

        let dest = self.base_dir.join(clean_filename);
        fs::write(&dest, bytes)?;
        Ok(dest)
    }

    pub fn read_file(&self, filename: &str) -> Result<Vec<u8>> {
        let clean_filename = Path::new(filename)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        let path = self.base_dir.join(clean_filename);
        let bytes = fs::read(path)?;
        Ok(bytes)
    }
}
