## ADDED Requirements

### Requirement: .http files use the dothttp language mode
VS Code SHALL associate files with the `.http` extension with the `dothttp` language identifier provided by the extension.

#### Scenario: Opening an http file
- **WHEN** a file with the `.http` extension is opened
- **THEN** VS Code SHALL display "dothttp" in the status bar language selector

---

### Requirement: TextMate grammar highlights HTTP syntax
The extension SHALL ship a TextMate grammar that highlights the following constructs in `.http` files:

- Request separator comments (`### Name`)
- HTTP method keywords (`GET`, `POST`, `PUT`, `DELETE`, `PATCH`, `HEAD`, `OPTIONS`)
- Request URL
- Header names and values
- Variable declarations (`@name = value`)
- Variable references (`{{name}}`)
- Request body (plain text; JSON bodies highlighted as embedded JSON)
- Line comments (`# comment`)

#### Scenario: Method keyword highlighted
- **WHEN** a `.http` file contains `GET https://api.example.com`
- **THEN** `GET` SHALL be highlighted with the method token scope

#### Scenario: Variable reference highlighted
- **WHEN** a `.http` file contains `{{baseUrl}}`
- **THEN** the variable reference SHALL be highlighted distinctly from surrounding text

#### Scenario: Request separator highlighted
- **WHEN** a `.http` file contains `### My Request`
- **THEN** the separator line SHALL be highlighted as a section heading
