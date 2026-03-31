## 1. Workspace Setup

- [x] 1.1 Add `dothttp-cli` to `[workspace.members]` in the root `Cargo.toml`
- [x] 1.2 Create `dothttp-cli/Cargo.toml` as a binary crate with dependencies: `dothttp-parser` (path), `dothttp-runner` (path), `clap` (with `derive` feature), `inquire`, `walkdir`, `colored`, `serde_json`, `tokio` (with `rt-multi-thread`, `macros`)
- [x] 1.3 Create `dothttp-cli/src/main.rs` with `#[tokio::main] async fn main()`

## 2. CLI Arguments

- [x] 2.1 Define a `Cli` struct using `clap::Parser` with a single optional positional argument `dir: Option<PathBuf>` (defaults to current directory)
- [x] 2.2 Validate that the provided directory exists; exit with a non-zero code and stderr message if not

## 3. File Discovery

- [x] 3.1 Create `src/discovery.rs` with a `discover` function that takes a `&Path` and returns `Vec<DiscoveredRequest>`
- [x] 3.2 In `discover`, use `walkdir::WalkDir` to recursively find all files with a `.http` extension
- [x] 3.3 For each file, call `dothttp_parser::parse_http_file`; on error print a warning to stderr and continue
- [x] 3.4 Define `DiscoveredRequest { label: String, request: RunnerRequest }` where label is `<relative_path>::<name_or_index>`
- [x] 3.5 Assign labels: use `request.name` if present, otherwise `#<n>` (1-based index within the file)
- [x] 3.6 If no `.http` files are found, print a message and exit with code 0

## 4. Request Selection

- [x] 4.1 Create `src/selection.rs` with a `select` function that takes `&[DiscoveredRequest]` and returns `Vec<usize>` (indices of selected requests)
- [x] 4.2 Detect TTY using `std::io::IsTerminal` on stdin; if non-TTY print "Non-interactive mode: running all requests" and return all indices
- [x] 4.3 In TTY mode, use `inquire::MultiSelect` to show labels and return the indices of chosen items
- [x] 4.4 If the user selects nothing, print "No requests selected." and exit with code 0

## 5. Response Display

- [x] 5.1 Create `src/display.rs` with a `print_result` function that takes a label `&str` and a `&RequestResult`
- [x] 5.2 Print a section header line with the request label (e.g. `─── api/users.http::Get Users ───`)
- [x] 5.3 On `Ok(HttpResponse)`: print the status code coloured by class (green 2xx, yellow 3xx, red 4xx/5xx) using the `colored` crate
- [x] 5.4 Print each response header as `Name: Value`
- [x] 5.5 If the body is non-empty, attempt `serde_json::from_str` and pretty-print with `serde_json::to_string_pretty`; on failure print raw body
- [x] 5.6 On `Err(RunnerError)`: print the error message in red

## 6. Main Orchestration

- [x] 6.1 Wire up `main`: parse args → discover → select → convert to `RunnerRequest` via `From` → run via `dothttp_runner::run` → display results
- [x] 6.2 After displaying all results, exit with code 1 if any result was `Err`, otherwise code 0

## 7. Tests

- [x] 7.1 In `discovery.rs` tests: create a temp dir with fixture `.http` files and assert correct label generation for named and unnamed requests
- [x] 7.2 In `discovery.rs` tests: assert a malformed `.http` file is skipped with a warning and other files are still returned
- [x] 7.3 In `display.rs` tests: assert `print_result` produces output containing the label, status code, and body for a known `RequestResult`

## 8. Verification

- [x] 8.1 Run `cargo build --workspace` and confirm zero errors
- [x] 8.2 Run `cargo test --workspace` and confirm all tests pass
- [x] 8.3 Run `cargo clippy --workspace` and resolve any warnings
