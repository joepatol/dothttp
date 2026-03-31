## ADDED Requirements

### Requirement: CLI accepts a --file flag to target a specific .http file
The CLI SHALL accept a `--file <path>` flag that, when provided, bypasses directory discovery and operates on the specified file directly.

#### Scenario: --file flag with valid path
- **WHEN** the user runs `dothttp-cli --file ./requests/users.http`
- **THEN** the CLI SHALL parse and use only that file, skipping directory scanning

#### Scenario: --file flag with non-existent path
- **WHEN** the user runs `dothttp-cli --file ./missing.http` and the file does not exist
- **THEN** the CLI SHALL exit with a non-zero code and print an error to stderr

#### Scenario: --file and dir argument both provided
- **WHEN** both `--file` and a positional directory argument are given
- **THEN** the CLI SHALL exit with a non-zero code and print an error indicating the options are mutually exclusive
