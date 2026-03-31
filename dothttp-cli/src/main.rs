mod discovery;
mod display;
mod selection;

use std::path::PathBuf;
use std::process;

use clap::Parser;

use discovery::{discover, discover_file};
use display::print_result;
use selection::{SelectionResult, select};

#[derive(Parser)]
#[command(name = "dothttp", about = "Run HTTP requests from .http files")]
struct Cli {
    /// Directory to scan for .http files (defaults to current directory)
    #[arg(conflicts_with = "file")]
    dir: Option<PathBuf>,

    /// Path to a specific .http file to run (mutually exclusive with dir)
    #[arg(long, conflicts_with = "dir")]
    file: Option<PathBuf>,

    /// Run only the request matching this name or "METHOD URL" identifier
    #[arg(long)]
    request: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let discovered = if let Some(file_path) = cli.file {
        if !file_path.exists() {
            eprintln!("error: file '{}' does not exist", file_path.display());
            process::exit(1);
        }
        if file_path.extension().and_then(|s| s.to_str()) != Some("http") {
            eprintln!("error: '{}' does not have a .http extension", file_path.display());
            process::exit(1);
        }
        discover_file(&file_path)
    } else {
        let dir = cli
            .dir
            .unwrap_or_else(|| std::env::current_dir().expect("cannot determine current directory"));

        if !dir.exists() {
            eprintln!("error: directory '{}' does not exist", dir.display());
            process::exit(1);
        }
        if !dir.is_dir() {
            eprintln!("error: '{}' is not a directory", dir.display());
            process::exit(1);
        }

        discover(&dir)
    };

    if discovered.is_empty() {
        println!("No .http requests found.");
        process::exit(0);
    }

    let indices = match select(&discovered, cli.request.as_deref()) {
        SelectionResult::Selected(idx) => idx,
        SelectionResult::NoneSelected => {
            println!("No requests selected.");
            process::exit(0);
        }
    };

    let selected: Vec<dothttp_runner::RunnerRequest> = indices
        .iter()
        .map(|&i| discovered[i].request.clone())
        .collect();

    let labels: Vec<&str> = indices.iter().map(|&i| discovered[i].label.as_str()).collect();

    let results = dothttp_runner::run(selected).await;

    let mut had_error = false;
    for (label, result) in labels.iter().zip(results.iter()) {
        if result.response.is_err() {
            had_error = true;
        }
        print_result(label, result);
    }

    if had_error {
        process::exit(1);
    }
}
