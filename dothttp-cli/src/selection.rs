use std::io::IsTerminal;

use crate::discovery::DiscoveredRequest;

pub enum SelectionResult {
    Selected(Vec<usize>),
    NoneSelected,
}

pub fn select(requests: &[DiscoveredRequest]) -> SelectionResult {
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
