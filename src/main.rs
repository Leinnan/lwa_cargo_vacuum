use std::{
    fs, io,
    time::{Duration, SystemTime},
};

use anyhow::Context;
use built_project::BuiltProject;
use chrono::{DateTime, Utc};
use clap::Parser;
use walkdir::WalkDir;

mod built_project;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let min_size = cli.minimal_size.unwrap_or(1);

    let max_time =
        SystemTime::now() - Duration::from_secs(cli.time_since_edit.unwrap_or(0) * 60 * 60);
    let path = cli.path.unwrap_or_else(|| ".".to_owned());

    let mut projects = WalkDir::new(path)
        .max_depth(cli.depth.unwrap_or(1))
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter_map(|entry| {
            BuiltProject::check_at_path(&entry)
                .context("Failed to check at path")
                .ok()
                .flatten()
        })
        .filter(|proj| proj.size > min_size && proj.last_modified < max_time)
        .collect::<Vec<BuiltProject>>();

    projects.sort_by(|a, b| b.size.cmp(&a.size));

    if projects.is_empty() {
        println!("No matching folders found, returning");
        return Ok(());
    }

    println!("{} projects:", projects.len());
    for p in &projects {
        let datetime: DateTime<Utc> = p.last_modified.into();
        println!(
            "\t\"{}\": {} MB, {}",
            &p.path
                .to_str()
                .context("Could not convert path to String")?,
            p.size,
            datetime.format("%Y-%m-%d %H:%M:%S")
        );
    }

    if cli.remove {
        let failed_to_remove_projects: Vec<(BuiltProject, io::Error)> = projects
            .into_iter()
            .filter_map(|proj| {
                if let Err(e) = fs::remove_dir_all(&proj.path) {
                    Some((proj, e))
                } else {
                    None
                }
            })
            .collect();

        if failed_to_remove_projects.is_empty() {
            println!("Projects removed!");
        } else {
            eprintln!("Failed to remove the following projects:");
            for (proj, error) in failed_to_remove_projects {
                eprintln!(
                    "\tProject at {} failed with error: {}",
                    proj.path.display(),
                    error
                );
            }
        }
    }

    Ok(())
}

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
