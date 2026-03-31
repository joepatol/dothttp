## Context

The dothttp project is a Rust-based HTTP file runner with three crates: `dothttp-parser` (parsing), `dothttp-runner` (async execution), and `dothttp-cli` (CLI interface). The CLI currently accepts a directory, discovers `.http` files, and uses an interactive `inquire::MultiSelect` prompt to choose which requests to run. In non-interactive mode it runs everything.

For VS Code integration the extension needs to trigger a single named request in a specific file without user interaction. This requires two parallel tracks: (1) extending the CLI to support non-interactive single-request execution, and (2) building the VS Code extension that shells out to that updated CLI.

## Goals / Non-Goals

**Goals:**
- Add `--file <path>` and `--request <name>` flags to `dothttp-cli` for non-interactive, targeted execution
- Surface a "Run" CodeLens above every HTTP request in `.http` files in VS Code
- Execute requests by shelling out to the updated `dothttp-cli` binary
- Display responses in a VS Code Webview panel with formatted output (status, headers, body)
- Provide TextMate-based syntax highlighting for `.http` files
- Expose extension settings: CLI binary path, default environment, request timeout

**Non-Goals:**
- Rewriting any Rust crates or embedding Rust via WASM/NAPI
- Building a GUI request editor (editing stays in the text file)
- Authentication/credential management UI
- Running all requests in a file from the extension (single-request focus initially)

## Decisions

### 1. Extend `dothttp-cli` with `--file` and `--request` flags

**Decision**: Add two new optional CLI arguments:
- `--file <path>`: Path to a specific `.http` file (bypasses directory discovery)
- `--request <name>`: Name of the request to run (matches the `### Name` separator comment); runs the named request only

When `--file` is provided without `--request`, run all requests in that file non-interactively. When `--request` is provided, run only the matching named request and exit non-zero if not found.

**Rationale**: The extension needs a stable, scriptable interface to target one request. The existing interactive prompt cannot be driven from a child process. Adding these flags keeps the CLI useful for both humans (interactive directory mode) and tools (non-interactive targeted mode) without breaking existing behavior.

### 2. Shell out to `dothttp-cli` rather than embedding Rust via NAPI or WASM

**Decision**: The extension spawns `dothttp-cli` as a child process using Node.js `child_process.spawn`.

**Rationale**: NAPI bindings require per-platform native compilation and significantly complicate packaging and CI. WASM loses access to the system network stack. Shelling out keeps the Rust codebase boundary clean and the extension portable — users need the binary on PATH or configured via settings.

**Alternative considered**: `@napi-rs` native module — rejected due to packaging complexity across Windows/macOS/Linux.

### 3. Response displayed in a Webview panel

**Decision**: Use a VS Code Webview panel to display responses with basic formatting (status code colored by class, headers table, body with syntax highlighting for JSON/XML).

**Rationale**: `OutputChannel` is append-only plain text, making it hard to style status codes or format structured bodies. A Webview gives full HTML/CSS control with minimal complexity (no JS framework — plain HTML templating in TypeScript).

**Alternative considered**: OutputChannel — simpler but poor UX for structured HTTP responses.

### 4. TextMate grammar for syntax highlighting

**Decision**: Ship a `.tmLanguage.json` TextMate grammar for `.http` files.

**Rationale**: TextMate grammars work without extension activation and are the standard for new file-type highlighting in VS Code. A semantic token provider would add activation complexity without meaningful benefit for `.http` syntax.

### 5. TypeScript with `esbuild` bundler

**Decision**: TypeScript source bundled with `esbuild` into a single `dist/extension.js`.

**Rationale**: `esbuild` is significantly faster than `webpack` and produces smaller bundles. No framework needed — the extension surface is small (CodeLens provider, command handlers, Webview).

### 6. CodeLens reads request names from document text

**Decision**: The CodeLens provider parses the open document's text to find `### <Name>` separators and places a "▶ Run" lens above each one. It passes the file path and request name to `dothttp-cli --file <path> --request <name>`.

**Rationale**: Parsing `### Name` from text is trivial with a regex and avoids needing a language server or tight coupling to the Rust parser. The CLI is the authority on actual parsing — the extension just needs to find the label.

## Risks / Trade-offs

- **Binary not found**: If `dothttp-cli` is not on PATH and the user hasn't configured the path, execution fails silently. → Mitigation: Check binary existence on extension activation; show a clear error notification with a link to settings.
- **CLI output format changes**: The extension parses CLI stdout to render the response. Future CLI changes could break parsing. → Mitigation: Add a `--output json` flag to the CLI (follow-up change) so the extension can consume structured output; for now parse the existing human-readable format.
- **Large response bodies**: Very large responses could freeze the Webview. → Mitigation: Truncate display beyond a configurable limit (default 1 MB) with a "View full response" button that opens a temp file.
- **Windows path handling**: `child_process.spawn` with paths containing spaces requires careful handling. → Mitigation: Use `cross-spawn` npm package for cross-platform child process spawning.
- **Request name collisions**: Multiple requests with the same name in one file — CLI behavior defines the outcome; extension just forwards the name.

## Migration Plan

1. Update `dothttp-cli` to add `--file` and `--request` flags (Rust changes, backward-compatible).
2. Create `dothttp-vscode/` at the repo root with extension scaffolding.
3. Implement CodeLens provider, command handlers, and Webview panel.
4. Add TextMate grammar and extension manifest contributions.
5. Test locally via VS Code "Run Extension" launch configuration.
6. Package with `vsce package`; publish to VS Code Marketplace.

## Open Questions

- Should `dothttp-cli` also gain `--output json` for machine-readable responses in this change, or defer to a follow-up?
- Marketplace publisher ID needs to be established before first publish.
- Should the extension auto-detect the binary from `target/debug/dothttp-cli` for local development convenience?
