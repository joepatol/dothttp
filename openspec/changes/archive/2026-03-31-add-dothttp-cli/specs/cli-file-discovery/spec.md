## ADDED Requirements

### Requirement: CLI accepts a directory path argument
The CLI SHALL accept a directory path as a positional argument. When omitted, it SHALL default to the current working directory.

#### Scenario: Explicit directory provided
- **WHEN** the user runs `dothttp-cli ./my-requests`
- **THEN** the CLI SHALL scan `./my-requests` for `.http` files

#### Scenario: No argument defaults to current directory
- **WHEN** the user runs `dothttp-cli` with no arguments
- **THEN** the CLI SHALL scan the current working directory for `.http` files

#### Scenario: Non-existent directory
- **WHEN** the user provides a path that does not exist
- **THEN** the CLI SHALL exit with a non-zero code and print an error message to stderr

---

### Requirement: CLI discovers all .http files recursively
The CLI SHALL walk the given directory recursively and collect every file with a `.http` extension.

#### Scenario: Nested directories
- **WHEN** `.http` files exist in subdirectories of the target
- **THEN** all such files SHALL be discovered and included

#### Scenario: No .http files found
- **WHEN** the target directory contains no `.http` files
- **THEN** the CLI SHALL print a message indicating no files were found and exit with code 0

---

### Requirement: Each request is labelled with file and name
Every discovered request SHALL be assigned a display label of the form `<relative_path>::<request_name>` where `<request_name>` is the request's `name` field or `#<n>` (1-based index) for unnamed requests.

#### Scenario: Named request
- **WHEN** a request has `name = Some("Get Users")`  in file `api/users.http`
- **THEN** its label SHALL be `api/users.http::Get Users`

#### Scenario: Unnamed request
- **WHEN** a request has `name = None` and is the second request in `api/users.http`
- **THEN** its label SHALL be `api/users.http::#2`

---

### Requirement: Parse errors in a file are reported but do not abort discovery
If a `.http` file cannot be parsed, the CLI SHALL print a warning to stderr and continue processing remaining files.

#### Scenario: One malformed file among several
- **WHEN** one file fails to parse and others succeed
- **THEN** the CLI SHALL warn about the malformed file and present requests from the remaining files
