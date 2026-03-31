## ADDED Requirements

### Requirement: Parse multiple requests from a single file
The parser SHALL accept a complete `.http` file and return a `HttpFile` containing an ordered list of all `Request` structs found in the file. Requests are delimited by lines beginning with `###`.

#### Scenario: Single request, no separator
- **WHEN** the input contains one request with no `###` line
- **THEN** `HttpFile.requests` contains exactly one `Request`

#### Scenario: Multiple requests separated by `###`
- **WHEN** the input contains two or more requests each preceded by a `###` line
- **THEN** `HttpFile.requests` contains the requests in source order

#### Scenario: Named request via `### Name`
- **WHEN** the `###` separator line has text after the marker (e.g. `### Fetch users`)
- **THEN** the corresponding `Request.name` is `Some("Fetch users")`

#### Scenario: Unnamed request via bare `###`
- **WHEN** the `###` separator line has no text after the marker
- **THEN** the corresponding `Request.name` is `None`

#### Scenario: Empty file
- **WHEN** the input is empty or contains only whitespace
- **THEN** `HttpFile.requests` is empty and no error is returned

#### Scenario: Separator at end of file with no following request
- **WHEN** a `###` line appears at the end with no subsequent request line
- **THEN** the trailing separator is ignored and no error is returned

#### Scenario: File cannot be parsed
- **WHEN** the input contains a malformed request (e.g. an unrecognised HTTP method or a missing URL)
- **THEN** `parse_http_file` returns a `ParseError` identifying the line number and the content that caused the failure
