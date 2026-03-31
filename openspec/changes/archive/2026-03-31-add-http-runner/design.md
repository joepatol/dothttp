## Context

The workspace currently has a single crate, `dothttp-parser`, which parses `.http` files into `Request` structs (with `name`, `method`, `url`, `headers`, `version`, `body`). There is no async runtime or HTTP client dependency in the project yet. The runner will be an entirely new workspace crate with its own request type — it accepts parser types via a `From` conversion but owns no parsing logic and does not expose parser types in its public API.

## Goals / Non-Goals

**Goals:**
- New `dothttp-runner` crate added to the workspace.
- The crate defines its own `RunnerRequest` struct — the runner's public API is independent of `dothttp-parser` types.
- `impl From<dothttp_parser::Request> for RunnerRequest` provided as the bridge so callers can easily convert parsed requests.
- Accept a `Vec<RunnerRequest>` and execute all requests concurrently.
- Return a `Vec<RequestResult>` that pairs each response (or error) back to its originating `RunnerRequest`.
- Clean async public API: a single `run(requests: Vec<RunnerRequest>) -> Vec<RequestResult>` function (or equivalent).
- Error-tolerant: a failing request must not abort others; errors are captured per-result.

**Non-Goals:**
- File I/O or `.http` file parsing — callers are responsible for parsing.
- Variable interpolation — already handled by `dothttp-parser` before conversion.
- Retry logic, timeouts, or advanced HTTP features in the initial implementation.
- CLI integration — this crate is a library.

## Decisions

### 1. Separate crate with its own request type
The runner is a new `dothttp-runner` crate added to `[workspace.members]`. It defines:

```rust
pub struct RunnerRequest {
    pub name: Option<String>,
    pub method: String,           // e.g. "GET", "POST"
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}
```

Using `String` for `method` (rather than re-exporting `HttpMethod`) means the runner has zero mandatory coupling to the parser's enum. Any caller — not just dothttp-parser users — can construct a `RunnerRequest` directly.

*Alternative considered:* Re-using `dothttp_parser::Request` directly in the runner API. Rejected because it would bleed parser internals into the runner's public interface and couple their release cycles.

### 2. `From<dothttp_parser::Request>` as the opt-in bridge
```rust
// in dothttp-runner, behind an optional `parser` feature or as a direct dev-dep
impl From<dothttp_parser::Request> for RunnerRequest {
    fn from(r: dothttp_parser::Request) -> Self {
        RunnerRequest {
            name: r.name,
            method: r.method.to_string(),  // HttpMethod implements Display
            url: r.url,
            headers: r.headers,
            body: r.body,
        }
    }
}
```
`dothttp-parser` is a `[dependencies]` entry in `dothttp-runner/Cargo.toml`. This keeps conversion logic in one place and avoids duplication in every caller.

*Alternative considered:* Putting the `From` impl in `dothttp-parser`. Rejected — the parser should not know about the runner.

### 3. `reqwest` as the HTTP client
`reqwest` is the idiomatic async Rust HTTP client, builds on `tokio`, and covers all HTTP methods/headers/body needed. Its `Client` is cheaply cloneable and safe to share across concurrent tasks.

*Alternative considered:* `hyper` directly. Rejected as too low-level for a first implementation.

### 4. Concurrent execution via `futures::future::join_all`
All requests are spawned as independent async futures and collected with `join_all`. This gives maximum concurrency. `join_all` preserves input order, so `results[i]` always corresponds to `requests[i]`.

*Alternative considered:* Sequential execution. Rejected — the requirement explicitly asks for async processing.

### 5. `RequestResult` for request/response pairing
```rust
pub struct RequestResult {
    pub request: RunnerRequest,
    pub response: Result<HttpResponse, RunnerError>,
}

pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
}
```
Keeping the original `RunnerRequest` inside `RequestResult` is the simplest way to guarantee request/response correlation regardless of execution order — no index bookkeeping needed by callers.

### 6. `RunnerError` is a crate-local error type
`RunnerError` wraps `reqwest::Error` and other execution failures, keeping `reqwest` types out of the runner's public API surface.

## Risks / Trade-offs

- **reqwest/tokio version alignment** — adds compile time and binary size. Mitigation: acceptable for first version; feature flags can be added later.
- **HTTP/2.0** — `RunnerRequest` does not carry a version field (intentional). `reqwest` negotiates HTTP version automatically. Callers that need explicit version control can extend `RunnerRequest`. Mitigation: document the omission.
- **Large response bodies** — bodies are collected as `String`. Mitigation: acceptable for initial scope; streaming can be added later.
- **`dothttp-runner` depends on `dothttp-parser`** — this is a one-way, opt-in dependency purely for the `From` impl. The parser never depends on the runner.

## Migration Plan

1. Add `dothttp-runner` to `[workspace.members]` in the root `Cargo.toml`.
2. Create `dothttp-runner/Cargo.toml` with dependencies: `dothttp-parser` (path dep), `reqwest`, `tokio`, `futures`.
3. Implement `RunnerRequest`, `HttpResponse`, `RunnerError`, `RequestResult`, `run()` function, and `From` impl.
4. Add tests using a mock HTTP server (e.g., `wiremock` or `mockito`).
5. No rollback needed — purely additive; existing parser crate is untouched.

## Open Questions

- Should the `From<dothttp_parser::Request>` impl live behind a Cargo feature flag to make `dothttp-parser` an optional dependency? Deferred — can be added when needed.
- Should `HttpResponse.body` be `Bytes` instead of `String` to handle binary responses? Deferred — string is sufficient for `.http` file use cases initially.
