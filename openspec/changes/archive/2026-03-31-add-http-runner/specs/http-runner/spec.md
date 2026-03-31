## ADDED Requirements

### Requirement: Runner accepts a list of requests
The runner SHALL accept a `Vec<RunnerRequest>` as its primary input and execute each request independently.

#### Scenario: Empty list
- **WHEN** `run` is called with an empty `Vec`
- **THEN** the runner SHALL return an empty `Vec<RequestResult>` without error

#### Scenario: Single request
- **WHEN** `run` is called with a single `RunnerRequest`
- **THEN** the runner SHALL return a `Vec<RequestResult>` with exactly one entry

#### Scenario: Multiple requests
- **WHEN** `run` is called with N requests
- **THEN** the runner SHALL return a `Vec<RequestResult>` with exactly N entries

---

### Requirement: Requests are executed asynchronously and concurrently
The runner SHALL execute all requests concurrently, not sequentially.

#### Scenario: Concurrent execution
- **WHEN** `run` is called with multiple requests
- **THEN** all requests SHALL be dispatched concurrently before any response is awaited

---

### Requirement: Each result is paired with its originating request
Every `RequestResult` SHALL contain the originating `RunnerRequest` so callers can correlate responses without relying on index bookkeeping.

#### Scenario: Result contains original request
- **WHEN** a request completes successfully
- **THEN** `RequestResult.request` SHALL be equal to the original `RunnerRequest` that was submitted

#### Scenario: Result order matches input order
- **WHEN** `run` returns
- **THEN** `results[i].request` SHALL correspond to `input[i]` for every valid index `i`

---

### Requirement: Individual request failures do not abort other requests
If one request fails (network error, timeout, etc.) the runner SHALL still collect results for all other requests.

#### Scenario: One failing request
- **WHEN** `run` is called with multiple requests and one request fails
- **THEN** the failing entry SHALL have `RequestResult.response = Err(RunnerError)`
- **AND** all other entries SHALL contain their own responses unaffected

---

### Requirement: RunnerRequest is a self-contained, parser-independent type
`RunnerRequest` SHALL be defined in `dothttp-runner` with no mandatory dependency on any type from `dothttp-parser`.

#### Scenario: Construct without parser
- **WHEN** a caller builds a `RunnerRequest` by filling in its fields directly (method string, url string, headers, optional body)
- **THEN** the runner SHALL execute it correctly without requiring `dothttp-parser` to be present in the caller's dependency tree

---

### Requirement: From<dothttp_parser::Request> conversion is provided
`dothttp-runner` SHALL provide `impl From<dothttp_parser::Request> for RunnerRequest` so users of the parser can convert without boilerplate.

#### Scenario: Convert parsed request
- **WHEN** a `dothttp_parser::Request` is converted via `RunnerRequest::from(parsed_request)`
- **THEN** all fields (name, method, url, headers, body) SHALL be transferred correctly to the resulting `RunnerRequest`

---

### Requirement: Successful HTTP response is captured in HttpResponse
A successful response SHALL be represented as `Ok(HttpResponse)` containing status code, response headers, and body.

#### Scenario: 200 OK response
- **WHEN** the remote server returns HTTP 200 with headers and a body
- **THEN** `HttpResponse.status` SHALL be `200`, headers SHALL be populated, and `body` SHALL contain the response body as a UTF-8 string

#### Scenario: Non-2xx response is not an error
- **WHEN** the remote server returns a 4xx or 5xx status code
- **THEN** `RequestResult.response` SHALL be `Ok(HttpResponse)` with the appropriate status code (not `Err`)
