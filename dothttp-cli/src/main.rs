mod discovery;
mod display;
mod selection;

use std::path::PathBuf;
use std::process;

use clap::Parser;

use discovery::discover;
use display::print_result;
use selection::{SelectionResult, select};

#[derive(Parser)]
#[command(name = "dothttp", about = "Run HTTP requests from .http files")]
struct Cli {
    /// Directory to scan for .http files (defaults to current directory)
    dir: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

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

    let discovered = discover(&dir);

    if discovered.is_empty() {
        println!("No .http files found in '{}'.", dir.display());
        process::exit(0);
    }

    let indices = match select(&discovered) {
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
