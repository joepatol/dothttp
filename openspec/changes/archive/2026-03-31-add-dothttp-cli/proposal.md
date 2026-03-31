## Why

The workspace has a parser and an async HTTP runner but no user-facing entry point — there is no way to actually use the tooling without writing Rust code. Adding a CLI ties the two libraries together and makes `dothttp` immediately usable from the terminal.

## What Changes

- New `dothttp-cli` crate added to the workspace.
- The CLI scans a directory for `.http` files, parses each with `dothttp-parser`, and presents the discovered requests in an interactive multi-select prompt.
- The user selects one or more requests; the CLI executes them via `dothttp-runner` and pretty-prints each response (status, headers, body).
- No changes to `dothttp-parser` or `dothttp-runner` public APIs.

## Capabilities

### New Capabilities

- `cli-file-discovery`: Scan a directory for `.http` files and collect all parsed requests, labelled by file and request name.
- `cli-request-selection`: Present an interactive multi-select prompt listing discovered requests; the user picks which ones to run.
- `cli-response-display`: Execute selected requests via the runner and pretty-print each response with status, headers, and formatted body.

### Modified Capabilities

<!-- No existing requirement changes. -->

## Impact

- New `dothttp-cli` crate added to `[workspace.members]`.
- Depends on `dothttp-parser` and `dothttp-runner` (both path deps).
- New binary dependencies: `clap` (arg parsing), `inquire` or `dialoguer` (interactive selection), `colored` or `owo-colors` (terminal colour).
- No breaking changes to any existing crate.
