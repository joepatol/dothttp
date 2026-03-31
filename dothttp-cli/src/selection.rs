use std::io::IsTerminal;

use crate::discovery::DiscoveredRequest;

pub enum SelectionResult {
    Selected(Vec<usize>),
    NoneSelected,
}

/// Select requests to run.
///
/// If `filter` is `Some(id)`, skip any prompt and return the single request
/// whose identifier matches `id`. Exits the process with an error if no match.
///
/// If `filter` is `None`, fall back to the interactive multi-select prompt
/// (or run-all in non-TTY mode).
pub fn select(requests: &[DiscoveredRequest], filter: Option<&str>) -> SelectionResult {
    if let Some(id) = filter {
        let indices: Vec<usize> = requests
            .iter()
            .enumerate()
            .filter(|(_, r)| r.identifier == id)
            .map(|(i, _)| i)
            .collect();

        if indices.is_empty() {
            let available: Vec<&str> = requests.iter().map(|r| r.identifier.as_str()).collect();
            eprintln!("error: no request found with identifier '{id}'");
            eprintln!("available identifiers:");
            for id in &available {
                eprintln!("  {id}");
            }
            std::process::exit(1);
        }

        return SelectionResult::Selected(indices);
    }

    if !std::io::stdin().is_terminal() {
        eprintln!("Non-interactive mode: running all requests");
        return SelectionResult::Selected((0..requests.len()).collect());
    }

    let labels: Vec<&str> = requests.iter().map(|r| r.label.as_str()).collect();

    let chosen = inquire::MultiSelect::new("Select requests to run:", labels)
        .prompt()
        .unwrap_or_default();

    if chosen.is_empty() {
        return SelectionResult::NoneSelected;
    }

    // Map chosen labels back to indices (preserving discovery order)
    let indices: Vec<usize> = requests
        .iter()
        .enumerate()
        .filter(|(_, r)| chosen.contains(&r.label.as_str()))
        .map(|(i, _)| i)
        .collect();

    SelectionResult::Selected(indices)
}
