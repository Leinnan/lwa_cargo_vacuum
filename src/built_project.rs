use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use anyhow::Context;
use walkdir::DirEntry;

#[derive(Debug)]
pub struct BuiltProject {
    pub path: PathBuf,
    pub size: u64,
    pub last_modified: std::time::SystemTime,
}

impl BuiltProject {
    pub fn new(path: &Path) -> anyhow::Result<Self> {
        let last_mod = Self::get_last_modified_entry(path)?;
        let last_modified = if let Some(entry) = last_mod {
            fs::metadata(entry.path())
                .context("Failed to get metadata for entry")?
                .modified()
                .context("Failed to get modification time for entry")?
        } else {
            fs::metadata(path)
                .context("Failed to get metadata")?
                .modified()
                .context("Failed to get modification time")?
        };

        Ok(Self {
            path: path.to_path_buf(),
            size: fs_extra::dir::get_size(path).unwrap_or(0) / (1024 * 1024),
            last_modified,
        })
    }

    pub fn check_at_path(entry: &DirEntry) -> anyhow::Result<Option<Self>> {
        fs::read_dir(entry.path()).context("Could not read directory")?;
        let entry = entry.path();
        let has_cargo = entry.join("Cargo.toml").exists();
        if has_cargo {
            return Ok(if entry.join("target").exists() {
                Some(Self::new(&entry.join("target"))?)
            } else {
                None
            });
        }
        let is_unity_project = entry
            .join("ProjectSettings")
            .join("ProjectSettings.asset")
            .exists();
        if is_unity_project {
            return Ok(if entry.join("Library").exists() {
                Some(Self::new(&entry.join("Library"))?)
            } else {
                None
            });
        }

        Ok(None)
    }

    /// Finds last modified directory entry
    fn get_last_modified_entry(dir_path: &Path) -> anyhow::Result<Option<fs::DirEntry>> {
        let mut last_modified_entry: Option<fs::DirEntry> = None;
        let mut last_modified_time: Option<std::time::SystemTime> = None;
        let Ok(entries) = fs::read_dir(dir_path) else {
            return Ok(None);
        };
        for entry in entries.flatten() {
            let metadata = entry.metadata()?;
            let modified_time = metadata.modified()?;

            let Some(last_time) = last_modified_time else {
                last_modified_time = Some(modified_time);
                last_modified_entry = Some(entry);
                continue;
            };
            if modified_time > last_time {
                last_modified_time = Some(modified_time);
                last_modified_entry = Some(entry);
            }
        }

        Ok(last_modified_entry)
    }
}
