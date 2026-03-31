## ADDED Requirements

### Requirement: Skip comment lines
The parser SHALL treat lines beginning with `//` or `#` (but NOT `###`) as comments and discard them without error. Comments may appear anywhere that a non-structural line is valid: before the first request, between requests, or between headers in a request segment.

#### Scenario: Comment before the first request
- **WHEN** the file starts with one or more `//` or `#` lines before any request
- **THEN** those lines are ignored and parsing continues normally

#### Scenario: Comment between requests
- **WHEN** a comment line appears between two `###`-separated requests
- **THEN** it is ignored and both requests are parsed correctly

#### Scenario: Comment between headers
- **WHEN** a comment line appears between header lines in a request segment
- **THEN** it is ignored and all surrounding headers are captured

#### Scenario: `###` is not treated as a comment
- **WHEN** a line starts with `###`
- **THEN** it is treated as a request separator, not a comment

#### Scenario: Inline comments are not supported
- **WHEN** a comment marker appears in the middle of a URL or header value (e.g. `GET http://example.com // comment`)
- **THEN** the comment marker is treated as part of the value, not as a comment
