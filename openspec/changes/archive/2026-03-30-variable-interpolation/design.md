## Context

`parse_http_file` already collects `Variable` structs into `HttpFile.variables` in source order. Request URLs, header values, and bodies are stored as raw strings that may contain `{{name}}` placeholders. Nothing currently substitutes them.

The interpolation pass needs to run after all variables have been collected (since a variable declared mid-file is available to all requests) and before the `HttpFile` is returned to the caller.

## Goals / Non-Goals

**Goals:**
- Substitute `{{name}}` references in request URLs, header values, and body content.
- Return `ParseError::UndefinedVariable` when a reference has no matching declaration.
- Keep interpolation transparent to callers — `parse_http_file` returns a fully resolved `HttpFile`.

**Non-Goals:**
- Chained variable references (a variable value may not reference another variable).
- Interpolation in header keys, HTTP method, or HTTP version strings.
- Escaping `{{` to produce a literal `{{` in output.
- Environment-level variable overrides or external variable sources.

## Decisions

### 1. Interpolation at the end of `parse_http_file`, not as a separate step

Interpolation is applied as the final pass inside `parse_http_file` before returning, rather than exposing a separate `interpolate(file: HttpFile)` function.

**Rationale:** Callers always want a resolved file; making them call a second function is error-prone. The variables are already fully collected by the time the last segment is parsed, so a single trailing pass is safe.

**Alternative considered:** A separate `interpolate` function in the public API. Rejected: adds surface area and requires callers to remember the extra call. Could be added later without breaking changes if needed.

### 2. Substitute in URL, header values, and body only

Header keys, method, and version strings are not interpolated.

**Rationale:** `{{}}` syntax in header names or methods is not part of any established `.http` tool convention and would silently corrupt the request structure. Limiting substitution to value positions keeps the surface predictable.

### 3. `ParseError::UndefinedVariable` for missing references

A `{{name}}` with no matching `@name = value` declaration returns `Err(ParseError::UndefinedVariable { name, line })`. The line number is the line of the field that contains the reference (URL line, header value line, etc.).

**Rationale:** Silently leaving `{{name}}` as-is would produce invalid requests (e.g., a URL containing `{{baseUrl}}`). Erroring eagerly gives the user actionable feedback.

**Alternative considered:** Leave undefined references as-is and let the runner fail at request time. Rejected: harder to debug; the parser already has full context to detect the problem.

### 4. Simple string-replace with winnow for reference scanning

References are found by scanning each value string for `{{` and `}}` delimiters using a winnow combinator loop, yielding a list of `(literal_segment, variable_name)` pairs that are then reassembled with substituted values.

**Rationale:** Re-using winnow keeps the parsing style consistent with the rest of the file. A simple `str::replace`-in-a-loop alternative would not handle multiple references in one string cleanly and would make it harder to pinpoint which reference is undefined.

**Alternative considered:** Regex. Rejected: adds a dependency; winnow is already present.

### 5. Undefined variable line reported as the field's source line

When a `{{name}}` is found in a URL, the line number in the error is the line of the request line. For a header value, it's the header line. For a body, it's the first body line.

**Rationale:** Exact character position would require threading offset information through the body (which is currently a single joined string). Line granularity is sufficient for a useful error message.

## Risks / Trade-offs

- **Multiple undefined variables in one file** → Only the first encountered is reported (fail-fast). A full list could be added later.
- **Variable declared after the reference** → All variables are collected first, so declaration order relative to usage doesn't matter. This is consistent with how tools like VS Code REST Client behave.
- **Body line number approximation** → The body is stored as a joined string without per-line offsets. The reported line is the first line of the body block, which may not be the exact line of the `{{reference}}`.
