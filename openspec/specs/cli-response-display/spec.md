### Requirement: Each response is displayed with its request label as a header
Before printing a response, the CLI SHALL print the request label as a section header so the user knows which response belongs to which request.

#### Scenario: Response header shown
- **WHEN** a response for `api/users.http::Get Users` is displayed
- **THEN** the output SHALL include a heading line identifying `api/users.http::Get Users`

---

### Requirement: Status line is printed with colour coding
The CLI SHALL print the HTTP status code and reason phrase on one line, coloured by status class.

#### Scenario: 2xx response
- **WHEN** the response status is in the 200–299 range
- **THEN** the status line SHALL be printed in green

#### Scenario: 3xx response
- **WHEN** the response status is in the 300–399 range
- **THEN** the status line SHALL be printed in yellow

#### Scenario: 4xx or 5xx response
- **WHEN** the response status is in the 400–599 range
- **THEN** the status line SHALL be printed in red

---

### Requirement: Response headers are printed as key-value lines
The CLI SHALL print each response header as `Key: Value` on its own line, below the status line.

#### Scenario: Headers displayed
- **WHEN** the response contains headers
- **THEN** each header SHALL appear as `<name>: <value>` on a separate line

---

### Requirement: Response body is pretty-printed when valid JSON
If the response body is valid UTF-8 JSON, the CLI SHALL print it indented. Otherwise it SHALL print the raw body text.

#### Scenario: JSON body
- **WHEN** the response body is valid JSON (e.g. `{"id":1}`)
- **THEN** the CLI SHALL print it indented with 2-space indentation

#### Scenario: Non-JSON body
- **WHEN** the response body is plain text or HTML
- **THEN** the CLI SHALL print it as-is

#### Scenario: Empty body
- **WHEN** the response body is empty
- **THEN** the CLI SHALL print nothing for the body section

---

### Requirement: Runner errors are displayed per-request without aborting output
If a request produced a `RunnerError`, the CLI SHALL print the error message in red and continue displaying remaining results.

#### Scenario: One request fails
- **WHEN** one request returns `Err(RunnerError)` and others succeed
- **THEN** the failed request SHALL display an error message in red
- **AND** all other responses SHALL be displayed normally

---

### Requirement: CLI exits with non-zero code if any request errored
After displaying all results, the CLI SHALL exit with code 1 if one or more requests returned a `RunnerError`.

#### Scenario: All requests succeed
- **WHEN** every selected request returns `Ok(HttpResponse)`
- **THEN** the exit code SHALL be 0

#### Scenario: At least one request fails
- **WHEN** at least one selected request returns `Err(RunnerError)`
- **THEN** the exit code SHALL be 1
