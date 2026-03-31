### Requirement: User can configure the CLI binary path
The extension SHALL expose a `dothttp.binaryPath` setting that specifies the absolute path to the `dothttp-cli` executable.

#### Scenario: Custom binary path configured
- **WHEN** `dothttp.binaryPath` is set to `/usr/local/bin/dothttp-cli`
- **THEN** the extension SHALL use that path to invoke the CLI

#### Scenario: Setting not configured
- **WHEN** `dothttp.binaryPath` is not set
- **THEN** the extension SHALL fall back to resolving `dothttp-cli` from the system PATH

---

### Requirement: User can configure the default environment
The extension SHALL expose a `dothttp.defaultEnvironment` setting that specifies which variable environment to pass to the CLI via `--env`.

#### Scenario: Default environment set
- **WHEN** `dothttp.defaultEnvironment` is set to `staging`
- **THEN** all CLI invocations SHALL include `--env staging`

#### Scenario: Default environment not set
- **WHEN** `dothttp.defaultEnvironment` is not configured
- **THEN** the CLI SHALL be invoked without an `--env` flag, using the CLI's own default behavior

---

### Requirement: Settings are validated on activation
On extension activation, the extension SHALL validate configured settings and surface actionable errors for invalid values.

#### Scenario: Binary path set to non-existent file
- **WHEN** `dothttp.binaryPath` points to a path that does not exist
- **THEN** the extension SHALL show a warning notification with a link to open settings
