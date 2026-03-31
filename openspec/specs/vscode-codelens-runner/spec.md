### Requirement: CodeLens shown above every request
The extension SHALL display a "▶ Run" CodeLens above every HTTP request in an open `.http` file, whether the request is named (preceded by `### <Name>`) or unnamed (identified by its `<METHOD> <URL>` line).

#### Scenario: File with named requests
- **WHEN** a `.http` file contains one or more `### <Name>` separators
- **THEN** a "▶ Run" CodeLens SHALL appear above each separator line

#### Scenario: File with unnamed requests
- **WHEN** a `.http` file contains requests not preceded by a `### <Name>` separator
- **THEN** a "▶ Run" CodeLens SHALL appear above the `<METHOD> <URL>` line of each such request

#### Scenario: Mixed named and unnamed requests
- **WHEN** a `.http` file contains both named and unnamed requests
- **THEN** every request SHALL have a "▶ Run" CodeLens regardless of whether it has a name separator

---

### Requirement: Clicking CodeLens executes the corresponding request
Clicking the "▶ Run" CodeLens SHALL invoke `dothttp.runRequest` with the current file path and the request identifier. Named requests use their name; unnamed requests use their `<METHOD> <URL>` string as the identifier.

#### Scenario: User clicks Run on a named request
- **WHEN** the user clicks the "▶ Run" CodeLens above `### Get Users`
- **THEN** `dothttp.runRequest` SHALL be invoked with the file path and the name `Get Users`

#### Scenario: User clicks Run on an unnamed request
- **WHEN** the user clicks the "▶ Run" CodeLens above `GET https://api.example.com/users`
- **THEN** `dothttp.runRequest` SHALL be invoked with the file path and the identifier `GET https://api.example.com/users`

---

### Requirement: CodeLens updates when the document changes
The CodeLens provider SHALL refresh when the document content changes so that newly added or removed requests are reflected immediately.

#### Scenario: New request added while file is open
- **WHEN** the user adds a new request (named or unnamed) and saves the file
- **THEN** a new "▶ Run" CodeLens SHALL appear above the new request

#### Scenario: Request removed while file is open
- **WHEN** the user deletes a request
- **THEN** the corresponding CodeLens SHALL disappear
