## ADDED Requirements

### Requirement: Substitute variable references in request fields
The parser SHALL replace every `{{name}}` reference in a request's URL, header values, and body with the value of the matching `@name` variable declaration before returning the `HttpFile`. Substitution SHALL happen for all requests in the file using the same shared variable map.

#### Scenario: URL interpolation
- **WHEN** a request URL contains `{{baseUrl}}` and `@baseUrl = https://api.example.com` is declared
- **THEN** the resolved `Request.url` equals `"https://api.example.com/path"` (with the reference replaced)

#### Scenario: Header value interpolation
- **WHEN** a header value contains `{{token}}` and `@token = abc123` is declared
- **THEN** the resolved header value equals `"Bearer abc123"`

#### Scenario: Body interpolation
- **WHEN** a request body contains `{{userId}}` and `@userId = 42` is declared
- **THEN** the resolved body contains `"42"` in place of `{{userId}}`

#### Scenario: Multiple references in one field
- **WHEN** a URL contains two references (e.g. `{{scheme}}://{{host}}/path`) and both variables are declared
- **THEN** both are substituted and the resulting URL is fully resolved

#### Scenario: Variable declared after usage
- **WHEN** a `{{name}}` reference appears in a request that precedes the `@name` declaration line
- **THEN** interpolation still succeeds (all variables are collected before substitution begins)

#### Scenario: Undefined variable reference
- **WHEN** a `{{name}}` reference appears but no `@name` declaration exists anywhere in the file
- **THEN** `parse_http_file` returns `Err(ParseError::UndefinedVariable { name, line })` where `name` is the variable name and `line` is the line number of the field containing the reference

#### Scenario: No references in file
- **WHEN** a file contains variable declarations but no `{{name}}` references in any request field
- **THEN** `parse_http_file` succeeds and returns the file unchanged

#### Scenario: Header keys are not interpolated
- **WHEN** a header key contains `{{name}}`
- **THEN** the key is returned as the literal string `"{{name}}"` without substitution

### Requirement: UndefinedVariable error is human-readable
`ParseError::UndefinedVariable` SHALL implement `Display` with a message that includes the variable name and the line number, e.g. `Parse error on line 3: undefined variable 'baseUrl'`.

#### Scenario: Display format
- **WHEN** `ParseError::UndefinedVariable { name: "token".to_string(), line: 5 }` is formatted with `Display`
- **THEN** the output contains both `"token"` and `"line 5"`
