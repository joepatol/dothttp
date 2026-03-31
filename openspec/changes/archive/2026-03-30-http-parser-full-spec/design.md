## Context

The `dothttp-parser` crate uses the `winnow` parsing combinator library to parse `.http` files into Rust data structures. The current implementation exposes a single `parse_http_request` function that parses exactly one request from a `&str` input. The body parser strips all whitespace, the API surface assumes one request per file, and there is no handling for comments or variable declarations.

The `.http` file format (as used by tools like the VS Code REST Client and JetBrains HTTP Client) supports:
- Multiple requests per file, separated by `###` lines (with an optional name after `###`)
- File-level variable declarations: `@name = value`
- Comment lines: `//` and `#` prefixed lines (anywhere a non-structural line may appear)
- Verbatim request bodies (JSON, XML, plain text, etc.) following a blank line

## Goals / Non-Goals

**Goals:**
- Parse a complete `.http` file into a `HttpFile` IR containing ordered variable declarations and requests.
- Preserve request body content exactly (verbatim, including internal whitespace and newlines).
- Skip comment lines wherever they appear.
- Store variable declarations in the IR for future use by an interpolation layer.
- Return structured, human-readable errors instead of opaque parser failures.
- Keep the `winnow` dependency; no new parsing dependencies.
- Maintain a clean public API on `lib.rs`.

**Non-Goals:**
- Variable interpolation (substituting `{{name}}` occurrences in URLs, headers, or bodies).
- Executing or validating HTTP requests.
- Supporting non-standard extensions beyond the common `.http` format subset.

## Decisions

### 1. Top-level IR: `HttpFile` struct

Introduce `HttpFile` as the return type of the new public entry-point `parse_http_file`:

```rust
pub struct HttpFile {
    pub variables: Vec<Variable>,
    pub requests: Vec<Request>,
}
```

**Rationale:** A `.http` file is more than one request. Callers (runner, CLI) need the full picture — including variables — at once. Returning a `Vec<Request>` alone would discard variable declarations and make future interpolation impossible without re-parsing.

**Alternative considered:** Return `(Vec<Variable>, Vec<Request>)` tuple. Rejected: a named struct is more ergonomic and easier to extend.

### 2. Dedicated error type: `ParseError`

Replace the raw `winnow::Result` error surface with a crate-level `ParseError` enum that implements `std::error::Error` and `std::fmt::Display`:

```rust
pub enum ParseError {
    UnknownMethod { method: String, line: usize },
    InvalidRequestLine { line: usize, content: String },
    InvalidHeader { line: usize, content: String },
    InvalidVariableDeclaration { line: usize, content: String },
    UnexpectedEndOfInput,
}
```

Display messages are human-readable, for example:
```
Parse error on line 5: unknown HTTP method 'FETCH'
Parse error on line 3: invalid header line 'ContentType application/json'
```

**Rationale:** Winnow's internal error types (`ErrMode`, `ContextError`) are opaque to callers and not suitable for display to end users. A typed `ParseError` enum allows callers (CLI, tests) to handle specific failure modes programmatically and always produce a clear, actionable message. Line numbers make errors immediately actionable.

**Alternative considered:** Use `anyhow::Error` or `Box<dyn Error>`. Rejected: loses structured variant matching; adds a dependency; line information would have to be embedded in strings.

**Alternative considered:** Keep winnow errors and convert via `.map_err()` at the boundary. This was partially the approach before — but without line tracking it still produces opaque messages.

### 3. Replace `parse_http_request` with `parse_http_file`

The old `parse_http_request` function is removed from the public API. A new `parse_http_file(input: &str) -> Result<HttpFile, ParseError>` is the sole entry point.

**Rationale:** The old signature (`&mut &str`) leaks parser internals and is only usable for a single request. A clean `&str → Result<HttpFile, ParseError>` boundary is easier to call and test.

**Alternative considered:** Keep `parse_http_request` as a lower-level utility. Rejected: it misleads callers and adds API surface with no benefit once `parse_http_file` exists.

### 4. Splitting on `###` markers

The file is split into segments on `###`-prefixed lines. Each segment is then parsed independently as a single request.

**Rationale:** The `###` marker is the canonical request separator. Processing segments in order naturally handles multiple requests and maps cleanly to winnow sub-parsers.

**Alternative considered:** One large winnow parser that accumulates requests. Viable but harder to reason about; the split-then-parse approach is simpler to test in isolation.

### 5. Verbatim body capture

The body is captured using `take_while(|_| true)` bounded by the next `###` segment boundary (handled by the pre-split approach). The body is stored as-is, without trimming internal whitespace.

**Rationale:** The current implementation's `replace(" ", "").replace("\n", "")` corrupts any structured body. Verbatim capture is correct and preserves whatever the user wrote.

**Note:** Only the final trailing newline from the segment boundary is trimmed.

### 6. Comment handling via line-level skip

Lines beginning with `//` or `#` (excluding `###`) are consumed and discarded at the line level before other parsers attempt to match structural content.

**Rationale:** Comments can appear before a request's request-line or interspersed with headers (per common tool behaviour). A line-level skip is simpler than threading comment awareness into every sub-parser.

### 7. Variable declarations parsed at file level

`@name = value` lines that appear before the first `###` or inside any request segment are parsed into `Variable` structs and collected into the top-level `HttpFile.variables` list.

**Rationale:** Tools like VS Code REST Client treat `@variable` lines at the file level as global. Keeping them in one flat list matches that model and keeps future interpolation simple.

## Risks / Trade-offs

- **Segment-split approach assumes `###` is only a separator** → If a body ever contained `###` at the start of a line, it would be misidentified as a separator. Mitigation: this matches behaviour of existing tools; document the constraint.
- **Removing `parse_http_request` is a breaking API change** → Any code calling the old function will not compile. Mitigation: the crate is not yet published; rename cleanly and update internal tests.
- **Verbatim body includes trailing newline from file** → Callers sending the body to an HTTP library will get an extra newline. Mitigation: trim only the final trailing newline of a segment's body portion.
- **Line numbers in errors require tracking** → The pre-split approach means each segment must carry its starting line offset. Mitigation: compute line offsets from the original input before splitting.

## Open Questions

- Should variables defined inside a request segment be request-scoped or still global? For now, treat all variables as file-scoped.
- Should an unknown HTTP method be an error or produce an `HttpMethod::Other(String)` variant? Defaulting to error for now; can be relaxed later.
