## Why

The current http-parser handles only a narrow subset of the `.http` file format: a single request per parse call, with broken body preservation (all whitespace stripped) and no support for comments or variable declarations. Before a runner can reliably execute requests, the parser must correctly model everything a `.http` file can express.

## What Changes

- **Parser returns `Vec<Request>`** instead of a single `Request` — a `.http` file may contain multiple requests separated by `###` delimiters.
- **Body content is preserved verbatim** — the current implementation strips all whitespace, corrupting JSON and other structured bodies.
- **Comment lines are skipped** — lines beginning with `//` or `#` (but not `###`) are treated as comments and ignored.
- **Variable declarations are parsed and stored** — lines of the form `@name = value` are captured in the IR; interpolation is out of scope for this change.
- **Models extended** — `models.rs` gains a `HttpFile` top-level struct and adjustments to `Variable` as needed.
- The `parse_http_request` public API is replaced by `parse_http_file` that returns a structured `HttpFile`.

## Capabilities

### New Capabilities

- `multi-request-parsing`: Parse a `.http` file containing one or more requests separated by `###` markers into an ordered list of `Request` structs.
- `variable-declaration`: Recognise and capture `@name = value` lines into the IR without performing interpolation.
- `comment-handling`: Skip comment lines (`//` and `#`) that appear anywhere a non-structural line may appear (before requests, between headers, etc.).
- `body-preservation`: Capture request bodies exactly as written, preserving all whitespace, newlines, and indentation.

### Modified Capabilities

## Impact

- `dothttp-parser/src/models.rs` — add `HttpFile` struct; `Variable` may be adjusted; `Request` body field type stays `Option<String>` but semantics change (verbatim content).
- `dothttp-parser/src/parsers/http_request.rs` — significant rewrite of parsing logic.
- `dothttp-parser/src/lib.rs` — public API changes; existing tests updated.
- No external dependencies added; uses existing `winnow` parser combinator library.
