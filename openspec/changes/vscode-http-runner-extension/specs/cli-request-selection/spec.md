## ADDED Requirements

### Requirement: CLI accepts a --request flag to run a single request non-interactively
The CLI SHALL accept a `--request <identifier>` flag that selects a single request by name (for named requests) or by `<METHOD> <URL>` (for unnamed requests), bypassing the interactive prompt entirely.

#### Scenario: --request matches a named request
- **WHEN** the user runs `dothttp-cli --file ./api.http --request "Get Users"`
- **THEN** only the request named `Get Users` SHALL be executed, without any interactive prompt

#### Scenario: --request matches an unnamed request by method and URL
- **WHEN** the user runs `dothttp-cli --file ./api.http --request "GET https://api.example.com/users"`
- **THEN** only the matching unnamed request SHALL be executed

#### Scenario: --request identifier not found
- **WHEN** the `--request` value does not match any request in the target file
- **THEN** the CLI SHALL exit with a non-zero code and print an error listing available request identifiers

#### Scenario: --request used without --file
- **WHEN** `--request` is provided but `--file` is not
- **THEN** the CLI SHALL apply the filter across all discovered requests in the directory and run the first match, or exit with an error if no match is found
