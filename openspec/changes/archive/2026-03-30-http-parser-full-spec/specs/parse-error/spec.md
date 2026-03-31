## ADDED Requirements

### Requirement: Structured parse error type
The parser SHALL expose a `ParseError` enum as the error type of `parse_http_file`. `ParseError` SHALL implement `std::error::Error` and `std::fmt::Display`, producing human-readable messages that include the line number and the content that triggered the error.

#### Scenario: Unknown HTTP method
- **WHEN** the request line contains an unrecognised method (e.g. `FETCH /users`)
- **THEN** `parse_http_file` returns `Err(ParseError::UnknownMethod { method: "FETCH", line: <n> })`
- **AND** displaying the error yields a message such as `Parse error on line 5: unknown HTTP method 'FETCH'`

#### Scenario: Invalid request line
- **WHEN** the request line is missing the URL or is otherwise malformed
- **THEN** `parse_http_file` returns `Err(ParseError::InvalidRequestLine { line: <n>, content: <raw line> })`
- **AND** the display message includes the line number and the offending content

#### Scenario: Invalid header line
- **WHEN** a header line does not contain a `:` separator (e.g. `Content-Type application/json`)
- **THEN** `parse_http_file` returns `Err(ParseError::InvalidHeader { line: <n>, content: <raw line> })`
- **AND** the display message includes the line number and the offending content

#### Scenario: Invalid variable declaration
- **WHEN** a line begins with `@` but does not match `@name = value`
- **THEN** `parse_http_file` returns `Err(ParseError::InvalidVariableDeclaration { line: <n>, content: <raw line> })`
- **AND** the display message includes the line number and the offending content

#### Scenario: Unexpected end of input
- **WHEN** the file ends mid-request (e.g. after the method but before the URL)
- **THEN** `parse_http_file` returns `Err(ParseError::UnexpectedEndOfInput)`
- **AND** displaying the error yields `Parse error: unexpected end of input`

#### Scenario: Error implements std::error::Error
- **WHEN** any `ParseError` variant is constructed
- **THEN** it can be used as a `Box<dyn std::error::Error>` without additional wrapping
