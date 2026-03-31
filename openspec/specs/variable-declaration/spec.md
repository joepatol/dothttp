## ADDED Requirements

### Requirement: Parse variable declarations into the IR
The parser SHALL recognise lines of the form `@name = value` and store them as `Variable` structs in `HttpFile.variables`. Declared variables are used for {{name}} interpolation in request URLs, header values, and bodies.

#### Scenario: Variable before any request
- **WHEN** the file starts with `@baseUrl = https://api.example.com` before any `###` separator
- **THEN** `HttpFile.variables` contains a `Variable { name: "baseUrl", value: "https://api.example.com" }`

#### Scenario: Variable inside a request segment
- **WHEN** a `@name = value` line appears between the `###` separator and the request line
- **THEN** the variable is collected into `HttpFile.variables`

#### Scenario: Multiple variables
- **WHEN** multiple `@name = value` lines appear throughout the file
- **THEN** all are present in `HttpFile.variables` in source order

#### Scenario: Value with spaces
- **WHEN** the variable value contains spaces (e.g. `@greeting = Hello World`)
- **THEN** the entire string after `= ` is captured as the value: `"Hello World"`

#### Scenario: Malformed variable line
- **WHEN** a line starts with `@` but does not match the `@name = value` pattern
- **THEN** `parse_http_file` returns a `ParseError::InvalidVariableDeclaration` with the line number and content
