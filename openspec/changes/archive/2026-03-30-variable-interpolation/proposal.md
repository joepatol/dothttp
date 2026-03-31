## Why

Variables declared with `@name = value` are currently stored in the IR but never substituted into request URLs, headers, or bodies — making them useless to any consumer. Interpolation is the final step that makes `.http` files with variables actually executable.

## What Changes

- `parse_http_file` applies interpolation automatically after parsing: all `{{name}}` references in request URLs, header values, and body content are replaced with the corresponding variable's value.
- A new `ParseError::UndefinedVariable` variant is added, returned when a `{{name}}` reference has no matching `@name` declaration.
- Variable values are treated as plain strings; they are not themselves interpolated (no chained references).

## Capabilities

### New Capabilities

- `variable-interpolation`: Substitute `{{name}}` references in request URLs, header values, and body content with the values of declared variables.

### Modified Capabilities

- `variable-declaration`: Remove the "interpolation is explicitly out of scope" clause now that interpolation is being delivered.

## Impact

- `dothttp-parser/src/parsers/http_file.rs` — add an interpolation pass at the end of `parse_http_file`.
- `dothttp-parser/src/error.rs` — add `UndefinedVariable { name: String, line: usize }` variant to `ParseError`.
- `dothttp-parser/src/lib.rs` — existing tests updated where needed; new interpolation tests added.
- No new dependencies.
