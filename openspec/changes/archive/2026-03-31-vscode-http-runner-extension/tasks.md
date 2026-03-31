## 1. CLI: Add --file flag

- [x] 1.1 Add `--file <path>` optional argument to `Cli` struct in `dothttp-cli/src/main.rs` using Clap
- [x] 1.2 Add validation: `--file` and positional `dir` are mutually exclusive; exit with error if both provided
- [x] 1.3 Add validation: file must exist and have `.http` extension; exit with error otherwise
- [x] 1.4 Update `main` to bypass `discover()` when `--file` is set and parse the single file directly
- [x] 1.5 Write tests for `--file` flag: valid path, missing file, mutually exclusive with dir

## 2. CLI: Add --request flag

- [x] 2.1 Add `--request <identifier>` optional argument to `Cli` struct
- [x] 2.2 Implement request matching logic: match by name for named requests, by `<METHOD> <URL>` for unnamed
- [x] 2.3 Update `selection` to accept an optional filter identifier and skip interactive prompt when provided
- [x] 2.4 Exit with non-zero and list available identifiers when `--request` value does not match any request
- [x] 2.5 Write tests for `--request` flag: named match, method+url match, no match, used without `--file`

## 3. VS Code Extension: Project Scaffolding

- [x] 3.1 Create `dothttp-vscode/` directory at repo root with `package.json` (VS Code extension manifest)
- [x] 3.2 Add `tsconfig.json` targeting ES2020, output to `dist/`
- [x] 3.3 Add `esbuild` build script that bundles `src/extension.ts` → `dist/extension.js`
- [x] 3.4 Add `cross-spawn` as a runtime dependency
- [x] 3.5 Configure `.vscodeignore` to exclude source files and dev dependencies from the packaged extension
- [x] 3.6 Add VS Code launch configuration (`.vscode/launch.json`) for "Run Extension" debugging

## 4. VS Code Extension: Language & Syntax Highlighting

- [x] 4.1 Create TextMate grammar file `syntaxes/dothttp.tmLanguage.json` with scopes for method keywords, URL, headers, separators, variables, and comments
- [x] 4.2 Register the `dothttp` language and associate it with `*.http` in `package.json` `contributes`
- [x] 4.3 Point the grammar contribution at `syntaxes/dothttp.tmLanguage.json`
- [x] 4.4 Manually verify syntax highlighting in VS Code for all major constructs

## 5. VS Code Extension: Settings

- [x] 5.1 Declare `dothttp.binaryPath` (string, default `""`) in `package.json` `contributes.configuration`
- [x] 5.2 Declare `dothttp.defaultEnvironment` (string, default `""`) in `package.json` `contributes.configuration`
- [x] 5.3 Implement `resolveBinaryPath()` helper: use `dothttp.binaryPath` if set, otherwise resolve `dothttp-cli` from PATH
- [x] 5.4 On activation, call `resolveBinaryPath()` and show a warning notification with "Open Settings" button if binary not found

## 6. VS Code Extension: Command Registration & CLI Invocation

- [x] 6.1 Register the `dothttp.runRequest` command in `package.json` `contributes.commands`
- [x] 6.2 Implement command handler in `src/extension.ts`: accept `filePath` and `requestIdentifier` arguments
- [x] 6.3 Implement `runCliRequest(filePath, identifier)` that spawns `dothttp-cli --file <path> --request <id>` using `cross-spawn`
- [x] 6.4 Pass `--env <value>` if `dothttp.defaultEnvironment` is configured
- [x] 6.5 Collect stdout and stderr from the child process; resolve with stdout on exit code 0, reject with stderr otherwise

## 7. VS Code Extension: Response Webview Panel

- [x] 7.1 Create `src/responsePanel.ts` with a `ResponsePanel` class that manages a `vscode.WebviewPanel`
- [x] 7.2 Implement `ResponsePanel.render(output: string)` that parses CLI stdout into status line, headers, and body sections
- [x] 7.3 Generate HTML for the Webview with sections for status (colored by 2xx/4xx/5xx), headers table, and body
- [x] 7.4 Pretty-print JSON bodies in the rendered output
- [x] 7.5 Truncate bodies exceeding 1 MB and show a truncation notice in the panel
- [x] 7.6 Reuse existing panel if already open (call `reveal()` instead of creating a new panel)
- [x] 7.7 On CLI error (non-zero exit), show `vscode.window.showErrorMessage` with the stderr content

## 8. VS Code Extension: CodeLens Provider

- [x] 8.1 Create `src/codeLensProvider.ts` implementing `vscode.CodeLensProvider`
- [x] 8.2 Implement document parsing to find all request positions: `### <Name>` separators and unnamed `<METHOD> <URL>` lines
- [x] 8.3 Return a `CodeLens` with command `dothttp.runRequest` for each discovered request position
- [x] 8.4 Register the CodeLens provider for the `dothttp` language in `src/extension.ts`
- [x] 8.5 Fire `onDidChangeCodeLenses` event on document change to refresh lenses

## 9. Integration & Packaging

- [x] 9.1 End-to-end manual test: open a `.http` file, click "▶ Run" on a named request, verify response panel
- [x] 9.2 End-to-end manual test: click "▶ Run" on an unnamed request, verify correct request is executed
- [x] 9.3 End-to-end manual test: set `dothttp.binaryPath` to an invalid path, verify warning notification appears
- [x] 9.4 Run `vsce package` and verify the `.vsix` installs cleanly in VS Code
