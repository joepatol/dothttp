## Context

The workspace has `dothttp-parser` (parses `.http` files into `Request` structs) and `dothttp-runner` (executes `Vec<RunnerRequest>` concurrently and returns `Vec<RequestResult>`). There is no user-facing binary. The CLI is a thin orchestration layer: discover → parse → select → run → display.

## Goals / Non-Goals

**Goals:**
- New `dothttp-cli` binary crate added to the workspace.
- Accept a directory path as a CLI argument; walk it recursively for `*.http` files.
- Parse every file with `dothttp-parser`; collect all requests labelled `<filename> / <request name or index>`.
- Present an interactive multi-select prompt so the user can pick which requests to run.
- Convert selected requests to `RunnerRequest` (via `From` impl) and pass to `dothttp-runner::run`.
- Pretty-print each response: status line (coloured by status class), headers table, body (syntax-highlighted JSON if applicable, otherwise raw text).
- Exit with a non-zero code if any request returned an error.

**Non-Goals:**
- Variable override from the command line — variables are resolved by the parser from the file.
- Watch mode / file reloading.
- Saving responses to disk.
- TUI dashboards or streaming output.

## Decisions

### 1. `clap` for argument parsing
`clap` is specified in the project tech stack. A single positional argument `<dir>` (default: current directory) is sufficient for v1. A `--pattern` flag can be added later.

### 2. `inquire` for interactive multi-select
`inquire` provides a polished `MultiSelect` prompt with fuzzy search out of the box. This is preferable to `dialoguer` because fuzzy filtering is critical when a directory contains many requests.

*Alternative considered:* `dialoguer`. Rejected — no built-in fuzzy search.

### 3. Request label format: `filename::name_or_index`
Each request is labelled `<relative_file_path>::<request_name>` (or `<relative_file_path>::#<n>` for unnamed requests). This is unambiguous when multiple files contain requests with the same name.

### 4. Response pretty-printing without a heavy dep
- Status line: `colored` crate for ANSI colour (green 2xx, yellow 3xx, red 4xx/5xx).
- Headers: printed as `Key: Value` lines.
- Body: attempt `serde_json::from_str` → pretty-print with `serde_json::to_string_pretty`; fall back to raw text. No syntax highlighting library needed.

*Alternative considered:* `bat` as a subprocess for syntax highlighting. Rejected — adds a runtime dep on an external binary.

### 5. Binary crate, not a lib+bin split
The CLI logic is entirely orchestration (glue code). There is nothing worth exposing as a library API, so a single `src/main.rs` is appropriate. Integration tests can invoke the binary via `std::process::Command`.

### 6. Async main via `#[tokio::main]`
`dothttp-runner::run` is async. The binary must be async; `#[tokio::main]` with the `rt-multi-thread` feature is the standard approach.

## Risks / Trade-offs

- **Large directories** — scanning a directory with thousands of `.http` files could be slow. Mitigation: `walkdir` is fast enough for typical usage; no buffering needed in v1.
- **Parse errors in one file should not abort the whole run** — if one `.http` file is malformed, warn and continue. Mitigation: collect errors per-file, print warnings, skip the file.
- **Non-TTY environments** — `inquire` requires a TTY; running in CI or piped output will panic. Mitigation: detect `!std::io::stdin().is_terminal()` and either error with a helpful message or fall back to running all discovered requests.
- **Body encoding** — `HttpResponse.body` is `String`; binary response bodies will be lossy. Mitigation: acceptable for v1; display `<binary body>` placeholder if UTF-8 decoding fails.

## Migration Plan

1. Add `dothttp-cli` to `[workspace.members]`.
2. Create binary crate with `Cargo.toml` and `src/main.rs`.
3. No changes to existing crates; purely additive.
4. No rollback needed.

## Open Questions

- Should the CLI support a `--select-all` / `--no-interactive` flag for scripted use? Deferred to v2.
- Should parse warnings (undefined variables, etc.) be printed to stderr or suppressed? Default: print to stderr with a warning prefix.
