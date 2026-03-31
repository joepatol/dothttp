## ADDED Requirements

### Requirement: Response displayed in a Webview panel
After a request executes, the extension SHALL open (or reuse) a Webview panel titled "dothttp Response" and render the HTTP response.

#### Scenario: Successful response received
- **WHEN** `dothttp-cli` exits with code 0 and produces output
- **THEN** the Webview panel SHALL open and display the status line, response headers, and body

#### Scenario: Panel already open
- **WHEN** a response panel is already open from a previous run
- **THEN** the existing panel SHALL be reused and its content replaced with the new response

---

### Requirement: Response panel shows status, headers, and body
The Webview SHALL display three sections: status line, headers, and body.

#### Scenario: Status line rendering
- **WHEN** a response is displayed
- **THEN** the HTTP status code and reason phrase SHALL be shown prominently at the top of the panel

#### Scenario: Headers rendering
- **WHEN** a response includes headers
- **THEN** all response headers SHALL be displayed as a key-value list

#### Scenario: Body rendering
- **WHEN** a response body is present
- **THEN** the body SHALL be displayed; JSON bodies SHALL be pretty-printed

---

### Requirement: Large response bodies are truncated in the panel
Response bodies exceeding 1 MB SHALL be truncated in the Webview display.

#### Scenario: Body exceeds limit
- **WHEN** the response body is larger than 1 MB
- **THEN** the panel SHALL display the first 1 MB of the body with a notice indicating truncation

---

### Requirement: CLI errors are shown as notifications
If `dothttp-cli` exits with a non-zero code or writes to stderr, the extension SHALL surface the error to the user.

#### Scenario: CLI exits non-zero
- **WHEN** `dothttp-cli` exits with a non-zero exit code
- **THEN** the extension SHALL show an error notification containing the stderr output
