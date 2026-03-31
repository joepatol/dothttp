## Why

The project currently has a parser that converts `.http` files into Rust data structures, but there is no way to actually execute those requests. Adding an async HTTP runner as a first-class, parser-independent component enables users to send requests and collect responses programmatically, while keeping the clean separation of concerns already established by the parser.

## What Changes

- Add a new `http-runner` crate/module that accepts a list of parsed `HttpRequest` structs and executes them asynchronously.
- Each response is paired with its originating request so callers can correlate results.
- The runner exposes a simple async API — no coupling to parsing, file I/O, or variable resolution.
- Results are collected into a structured type that bundles the original request alongside the response (or error).

## Capabilities

### New Capabilities

- `http-runner`: Async HTTP runner that accepts a list of requests, executes them concurrently, and returns a list of paired request/response results.

### Modified Capabilities

<!-- No existing requirement changes. -->

## Impact

- New module/crate added under the project (e.g., `runner/` or `http-runner/`).
- Depends on the existing parsed `HttpRequest` type from the parser — read-only dependency; parser internals are not changed.
- Introduces an async runtime dependency (e.g., `reqwest` + `tokio`).
- No breaking changes to the parser or any existing public API.
