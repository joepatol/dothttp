## ADDED Requirements

### Requirement: Extension activates for .http files
The extension SHALL activate when a `.http` file is opened in VS Code.

#### Scenario: Opening an http file
- **WHEN** the user opens a file with the `.http` extension
- **THEN** the extension SHALL activate and register all commands and providers

#### Scenario: No http file open
- **WHEN** VS Code starts without any `.http` file open
- **THEN** the extension SHALL NOT activate until a `.http` file is opened

---

### Requirement: Extension registers run-request command
The extension SHALL register a `dothttp.runRequest` command that accepts a file path and request name as arguments.

#### Scenario: Command invoked with valid arguments
- **WHEN** `dothttp.runRequest` is called with a file path and request name
- **THEN** the extension SHALL execute the request using the configured CLI binary

#### Scenario: Command invoked without arguments
- **WHEN** `dothttp.runRequest` is called with no arguments
- **THEN** the extension SHALL show an error notification indicating arguments are required

---

### Requirement: Extension warns when CLI binary is not found
On activation, the extension SHALL verify the configured CLI binary exists and is executable.

#### Scenario: Binary missing at activation
- **WHEN** the extension activates and the CLI binary cannot be found at the configured path or on PATH
- **THEN** the extension SHALL show a warning notification with a button to open extension settings
