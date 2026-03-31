## 1. Workspace Setup

- [x] 1.1 Add `dothttp-runner` to `[workspace.members]` in the root `Cargo.toml`
- [x] 1.2 Create `dothttp-runner/` directory with `Cargo.toml` — add dependencies: `dothttp-parser` (path), `reqwest` (with default features), `tokio` (with `rt-multi-thread`, `macros`), `futures`
- [x] 1.3 Create `dothttp-runner/src/lib.rs` as the crate root with module declarations

## 2. Core Types

- [x] 2.1 Define `RunnerRequest` struct (`name: Option<String>`, `method: String`, `url: String`, `headers: Vec<(String, String)>`, `body: Option<String>`) in `src/models.rs`
- [x] 2.2 Define `HttpResponse` struct (`status: u16`, `headers: Vec<(String, String)>`, `body: String`) in `src/models.rs`
- [x] 2.3 Define `RequestResult` struct containing `request: RunnerRequest` and `response: Result<HttpResponse, RunnerError>` in `src/models.rs`
- [x] 2.4 Define `RunnerError` enum wrapping `reqwest::Error` and a string message variant in `src/error.rs`; implement `std::error::Error` and `Display`

## 3. Parser Conversion

- [x] 3.1 Implement `From<dothttp_parser::Request> for RunnerRequest` — map `method` via `HttpMethod::to_string()`, copy all other fields directly
- [x] 3.2 Verify `HttpMethod` implements `Display` in `dothttp-parser`; add the impl if missing

## 4. Runner Implementation

- [x] 4.1 Create `src/runner.rs` with an async `run(requests: Vec<RunnerRequest>) -> Vec<RequestResult>` function
- [x] 4.2 Inside `run`, build a shared `reqwest::Client` and map each `RunnerRequest` to an async future that executes the request and returns a `RequestResult`
- [x] 4.3 Use `futures::future::join_all` to drive all futures concurrently and collect results
- [x] 4.4 In the per-request future: build the `reqwest::RequestBuilder` from method, url, headers, and body; await it; map the response into `HttpResponse` (status, headers, body text)
- [x] 4.5 Wrap any `reqwest::Error` in `RunnerError`; ensure a single request failure does not affect others
- [x] 4.6 Re-export `run`, `RunnerRequest`, `HttpResponse`, `RequestResult`, `RunnerError` from `lib.rs`

## 5. Tests

- [x] 5.1 Add `wiremock` (or `mockito`) as a `[dev-dependencies]` entry
- [x] 5.2 Write a test: `run` with an empty list returns an empty result
- [x] 5.3 Write a test: single GET request returns correct status, headers, and body
- [x] 5.4 Write a test: multiple requests execute and all results are returned with correct request pairing (verify `result[i].request == input[i]`)
- [x] 5.5 Write a test: one failing request (bad URL / connection refused) produces `Err(RunnerError)` while other requests succeed
- [x] 5.6 Write a test: `From<dothttp_parser::Request>` conversion preserves all fields correctly
- [x] 5.7 Write a test: non-2xx responses (e.g. 404, 500) are returned as `Ok(HttpResponse)` not `Err`

## 6. Verification

- [x] 6.1 Run `cargo build --workspace` and confirm zero errors
- [x] 6.2 Run `cargo test --workspace` and confirm all tests pass
- [x] 6.3 Run `cargo clippy --workspace` and resolve any warnings
