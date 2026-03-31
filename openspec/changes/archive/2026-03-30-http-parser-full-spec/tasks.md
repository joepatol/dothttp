## 1. Models

- [x] 1.1 Add `HttpFile` struct to `models.rs` with `variables: Vec<Variable>` and `requests: Vec<Request>` fields
- [x] 1.2 Verify `Variable` struct has `name: String` and `value: String` fields (extend if needed)
- [x] 1.3 Derive `Debug` (and `PartialEq` where appropriate) on all new/modified structs

## 2. Error Type

- [x] 2.1 Create `error.rs` in `dothttp-parser/src/` and define the `ParseError` enum with variants: `UnknownMethod`, `InvalidRequestLine`, `InvalidHeader`, `InvalidVariableDeclaration`, `UnexpectedEndOfInput`
- [x] 2.2 Implement `std::fmt::Display` for `ParseError` with human-readable messages including line numbers
- [x] 2.3 Implement `std::error::Error` for `ParseError`
- [x] 2.4 Export `ParseError` from `lib.rs`

## 3. File-Level Parsing

- [x] 3.1 Create `parse_http_file(input: &str) -> Result<HttpFile, ParseError>` as the new public entry point in `parsers/`
- [x] 3.2 Implement logic to split the input into segments on `###`-prefixed lines, recording the starting line number of each segment
- [x] 3.3 Implement parsing of `@name = value` variable declaration lines (file-level and within segments) into `Variable` structs
- [x] 3.4 Implement comment-line skipping for lines beginning with `//` or `#` (excluding `###`)

## 4. Per-Request Parsing

- [x] 4.1 Implement `parse_request_segment(segment: &str, line_offset: usize) -> Result<Option<Request>, ParseError>` that parses one `###`-delimited block
- [x] 4.2 Update the request-line parser to propagate `ParseError::UnknownMethod` and `ParseError::InvalidRequestLine` with correct line numbers
- [x] 4.3 Update the header parser to propagate `ParseError::InvalidHeader` with correct line numbers
- [x] 4.4 Fix body parser to capture content verbatim (remove current whitespace-stripping logic)
- [x] 4.5 Ensure trailing newlines at the end of a body segment are trimmed, but internal whitespace is preserved

## 5. Public API Cleanup

- [x] 5.1 Remove `parse_http_request` from the public API (`lib.rs` and `parsers/mod.rs`)
- [x] 5.2 Export `parse_http_file`, `HttpFile`, `Request`, `Variable`, `HttpMethod`, `HttpVersion`, and `ParseError` from `lib.rs`

## 6. Tests

- [x] 6.1 Update existing tests in `lib.rs` to use `parse_http_file` and assert on `HttpFile.requests[0]`
- [x] 6.2 Add test: file with multiple requests returns correct count and order
- [x] 6.3 Add test: named and unnamed requests populate `Request.name` correctly
- [x] 6.4 Add test: `@variable` declarations are collected into `HttpFile.variables`
- [x] 6.5 Add test: comment lines (`//` and `#`) are ignored without error
- [x] 6.6 Add test: JSON body is preserved verbatim (regression against whitespace stripping)
- [x] 6.7 Add test: unknown HTTP method returns `ParseError::UnknownMethod` with correct line
- [x] 6.8 Add test: invalid header line returns `ParseError::InvalidHeader` with correct line
- [x] 6.9 Add test: `ParseError` display messages are human-readable and include line numbers
- [x] 6.10 Add `.http` test fixture files covering multi-request and variable scenarios
