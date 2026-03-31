## ADDED Requirements

### Requirement: Capture request body verbatim
The parser SHALL capture the request body exactly as written in the file, preserving all internal whitespace, indentation, and newlines. The body begins after the blank line that follows the last header, and ends at the next `###` separator or end of file.

#### Scenario: JSON body preserved with indentation
- **WHEN** the body is a JSON object with indented fields
- **THEN** `Request.body` contains the JSON with all original whitespace intact

#### Scenario: No body
- **WHEN** there is no blank line after the headers (or the file ends immediately after headers)
- **THEN** `Request.body` is `None`

#### Scenario: Empty body after blank line
- **WHEN** a blank line is present after the headers but no content follows before the next separator or EOF
- **THEN** `Request.body` is `None`

#### Scenario: Trailing newline trimmed
- **WHEN** the body ends with one or more newlines introduced by the segment boundary
- **THEN** only the trailing newline(s) at the very end of the body string are trimmed; internal newlines are preserved

#### Scenario: Plain-text body
- **WHEN** the body is a plain text string with no special structure
- **THEN** `Request.body` is `Some` containing the text verbatim
