use std::{
    fs,
    path::PathBuf,
    time::{Duration, SystemTime},
};

use chrono::{DateTime, Utc};
use clap::Parser;
use dpc_pariter::IteratorExt;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(version, about, long_about = Some("Simple CLI tool for cleaning up old target folders. By default just lists founded directories, use remove flag to remove founded directories."))]
struct Cli {
    /// directories search depth
    #[arg(short, long)]
    depth: Option<usize>,
    /// Relative path to operate on
    path: Option<String>,
    /// Minimal size of target folder in MB
    #[arg(long)]
    minimal_size: Option<u64>,
    /// Minimal time since last edit in hours
    #[arg(long)]
    time_since_edit: Option<u64>,
    /// removes target dirs matching requirements
    #[arg(short, long)]
    remove: bool,
}

#[derive(Debug)]
pub struct BuildedProject {
    pub path: PathBuf,
    pub size: u64,
    pub last_modified: std::time::SystemTime,
}

fn main() {
    let cli = Cli::parse();
    let min_size = cli.minimal_size.unwrap_or(1);

    let max_time =
        SystemTime::now() - Duration::from_secs(cli.time_since_edit.unwrap_or(0) * 60 * 60);
    let path = cli.path.unwrap_or(".".to_owned());
    let mut projects: Vec<BuildedProject> = WalkDir::new(path)
        .max_depth(cli.depth.unwrap_or(1))
        .into_iter()
        .parallel_filter(|entry| entry.is_ok())
        .parallel_map(|e| e.unwrap())
        .parallel_filter(|entry| {
            let Ok(dir_entries) = fs::read_dir(entry.path()) else {
                return false;
            };
            let has_target = dir_entries.into_iter().any(|e| {
                let Ok(entry) = e else {
                    return false;
                };
                return entry.file_type().unwrap().is_dir()
                    && entry.file_name().to_str().eq(&Some("target"));
            });
            if !has_target {
                return false;
            }
            let has_cargo = fs::read_dir(entry.path()).unwrap().any(|e| {
                let Ok(entry) = e else {
                    return false;
                };
                return entry.file_type().unwrap().is_file()
                    && entry.file_name().to_str().eq(&Some("Cargo.toml"));
            });
            has_cargo
        })
        .parallel_map(|e| BuildedProject {
            path: e.path().to_path_buf(),
            size: fs_extra::dir::get_size(e.path()).unwrap_or(0) / (1024 * 1024),
            last_modified: e.metadata().unwrap().modified().unwrap(),
        })
        .parallel_filter(move |proj| proj.size > min_size && proj.last_modified < max_time)
        .collect();
    projects.sort_by(|a, b| b.size.cmp(&a.size));
    if projects.is_empty() {
        println!("No matching folders found, returning");
        return;
    }
    println!("{} projects:", projects.len());
    for p in &projects {
        let datetime: DateTime<Utc> = p.last_modified.into();
        println!(
            "\t\"{}\": {} MB, {}",
            &p.path.to_str().unwrap(),
            p.size,
            datetime.format("%Y-%m-%d %H:%M:%S")
        );
    }
    if cli.remove {
        let failed_to_remove_projects: Vec<BuildedProject> = projects
            .into_iter()
            .parallel_filter(|e| fs::remove_dir_all(&e.path).is_err())
            .collect();
        if !failed_to_remove_projects.is_empty() {
            eprintln!(
                "Failed to remove projects! \n\t{:?}",
                failed_to_remove_projects
            );
        } else {
            println!("Projects removed!");
        }
    }
}
