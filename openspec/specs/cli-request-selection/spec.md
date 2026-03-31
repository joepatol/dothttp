### Requirement: Interactive multi-select prompt is shown
The CLI SHALL present an interactive multi-select prompt listing all discovered requests by label when running in a TTY.

#### Scenario: Multiple requests available
- **WHEN** discovery finds N requests and the user is on a TTY
- **THEN** the CLI SHALL display a multi-select prompt listing all N request labels

#### Scenario: User selects a subset
- **WHEN** the user checks 2 out of 5 requests and confirms
- **THEN** only the 2 selected requests SHALL be executed

#### Scenario: User selects none and confirms
- **WHEN** the user confirms without selecting any request
- **THEN** the CLI SHALL print a message ("No requests selected") and exit with code 0

---

### Requirement: Non-TTY environments run all requests without prompting
When stdin is not a TTY (e.g. CI, piped input), the CLI SHALL skip the interactive prompt and execute all discovered requests automatically.

#### Scenario: Non-TTY detected
- **WHEN** the CLI is invoked with stdin not attached to a TTY
- **THEN** all discovered requests SHALL be executed without a prompt
- **AND** the CLI SHALL print a message indicating non-interactive mode

---

### Requirement: Selected requests are passed to the runner in label order
The CLI SHALL execute requests in the order they appear in the selection list (top to bottom), preserving the discovery order.

#### Scenario: Order preserved
- **WHEN** the user selects requests in an arbitrary order from the prompt
- **THEN** execution results SHALL be displayed in the original discovery order, not the selection order

---

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
