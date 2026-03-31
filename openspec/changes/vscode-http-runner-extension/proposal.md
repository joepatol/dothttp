## Why

The dothttp HTTP file runner is a powerful CLI tool, but developers spend most of their time in VS Code. A native extension brings request execution directly into the editor, eliminating context-switching and enabling a tight feedback loop without leaving the IDE.

## What Changes

- New VS Code extension package (`dothttp-vscode`) that integrates with the existing `dothttp-cli` binary
- CodeLens annotations above each HTTP request in `.http` files with a "Run" action
- Output panel or webview to display HTTP responses inline within VS Code
- Extension commands for running individual requests and all requests in a file
- Syntax highlighting for `.http` files within VS Code
- Extension settings to configure the path to the `dothttp-cli` binary and environment variables

## Capabilities

### New Capabilities

- `vscode-extension-host`: The VS Code extension package structure, activation, and command registration
- `vscode-codelens-runner`: CodeLens provider that annotates each HTTP request with a run action
- `vscode-response-display`: Webview or output channel panel for rendering HTTP responses
- `vscode-syntax-highlighting`: TextMate grammar for `.http` file syntax highlighting in VS Code
- `vscode-settings`: Extension configuration (binary path, default environment, etc.)

### Modified Capabilities

## Impact

- New top-level package `dothttp-vscode/` (TypeScript/Node.js, not Rust)
- Depends on the `dothttp-cli` binary being built and available on PATH or configured path
- Introduces a `package.json`, `tsconfig.json`, and VS Code extension manifest (`vscode.d.ts` / `package.json` contributes)
- No changes to existing Rust crates; the extension shells out to `dothttp-cli`
- Requires VS Code Extension API (`vscode` npm package) as a peer dependency
