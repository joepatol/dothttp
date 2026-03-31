## 1. Error Type

- [x] 1.1 Add `UndefinedVariable { name: String, line: usize }` variant to `ParseError` in `error.rs`
- [x] 1.2 Implement `Display` for the new variant: `"Parse error on line {line}: undefined variable '{name}'"`

## 2. Reference Scanner (winnow)

- [x] 2.1 Implement a winnow-based `scan_references(input: &str) -> Vec<(usize, &str)>` (or equivalent) that returns the byte offsets and names of all `{{name}}` references found in a string
- [x] 2.2 Implement `interpolate_str(value: &str, vars: &HashMap<&str, &str>, line: usize) -> Result<String, ParseError>` that replaces every `{{name}}` with its value, returning `ParseError::UndefinedVariable` for any missing key

## 3. Interpolation Pass

- [x] 3.1 After all segments are parsed inside `parse_http_file`, build a `HashMap<&str, &str>` from `HttpFile.variables`
- [x] 3.2 For each `Request`, interpolate `url` using `interpolate_str`
- [x] 3.3 For each `Request`, interpolate each header value (second element of the tuple) using `interpolate_str`
- [x] 3.4 For each `Request`, interpolate `body` (if `Some`) using `interpolate_str`

## 4. Tests

- [x] 4.1 Add test: `{{baseUrl}}` in URL is replaced with the declared variable value
- [x] 4.2 Add test: `{{token}}` in a header value is replaced correctly
- [x] 4.3 Add test: `{{userId}}` in a JSON body is replaced correctly
- [x] 4.4 Add test: multiple references in a single URL are all replaced
- [x] 4.5 Add test: variable declared after the request that uses it is still resolved
- [x] 4.6 Add test: undefined variable returns `ParseError::UndefinedVariable` with correct name and line
- [x] 4.7 Add test: file with no `{{}}` references is returned unchanged
- [x] 4.8 Add test: header keys containing `{{name}}` are NOT interpolated
- [x] 4.9 Add test: `ParseError::UndefinedVariable` display message includes name and line number
- [x] 4.10 Add fixture file `interpolation.http` with variables and references for integration testing
