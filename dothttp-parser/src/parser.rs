use std::collections::HashMap;

use winnow::Result as WResult;
use winnow::Parser;
use winnow::ascii::{space0, space1};
use winnow::combinator::{alt, opt, preceded, terminated};
use winnow::token::{literal, take_until, take_while};

use crate::error::ParseError;
use crate::interpolation::RequestMeta;
use crate::models::{HttpFile, HttpMethod, HttpVersion, Request, Variable};

// ── Public entry point ───────────────────────────────────────────────────────

/// Parse a complete `.http` file into an `HttpFile` containing all variable
/// declarations and requests found in the input.
pub fn parse_http_file(input: &str) -> Result<HttpFile, ParseError> {
    if input.trim().is_empty() {
        return Ok(HttpFile {
            variables: vec![],
            requests: vec![],
        });
    }

    let lines: Vec<(usize, &str)> = input
        .lines()
        .enumerate()
        .map(|(i, l)| (i + 1, l))
        .collect();

    let mut variables: Vec<Variable> = Vec::new();
    let mut raw_requests: Vec<(Request, RequestMeta)> = Vec::new();

    for segment in split_segments(&lines) {
        if segment.is_empty() {
            continue;
        }

        let (_line_num, first_line) = segment[0];

        if first_line.starts_with("###") {
            let name = parse_separator_name(first_line);
            let (seg_vars, maybe_req) = parse_request_segment(&segment[1..], name)?;
            variables.extend(seg_vars);
            if let Some(req_and_meta) = maybe_req {
                raw_requests.push(req_and_meta);
            }
        } else {
            // Segment without a ### separator: may contain variables and/or an unnamed request.
            let (seg_vars, maybe_req) = parse_request_segment(&segment, None)?;
            variables.extend(seg_vars);
            if let Some(req_and_meta) = maybe_req {
                raw_requests.push(req_and_meta);
            }
        }
    }

    // Build variable map and apply interpolation now that all variables are known.
    let var_map: HashMap<&str, &str> = variables
        .iter()
        .map(|v| (v.name.as_str(), v.value.as_str()))
        .collect();

    let requests = raw_requests
        .into_iter()
        .map(|(req, meta)| req.interpolate(&var_map, &meta))
        .collect::<Result<Vec<_>, ParseError>>()?;

    Ok(HttpFile { variables, requests })
}

// ── Segment splitting ────────────────────────────────────────────────────────

/// Split enumerated lines into segments at `###` boundaries.
/// Each `###` line starts a new segment.
fn split_segments<'a>(lines: &'a [(usize, &'a str)]) -> Vec<Vec<(usize, &'a str)>> {
    let mut segments: Vec<Vec<(usize, &str)>> = Vec::new();
    let mut current: Vec<(usize, &str)> = Vec::new();

    for &item in lines {
        if item.1.starts_with("###") && !current.is_empty() {
            segments.push(std::mem::take(&mut current));
        }
        current.push(item);
    }
    if !current.is_empty() {
        segments.push(current);
    }
    segments
}

/// Extract the optional request name that follows the `###` marker.
fn parse_separator_name(line: &str) -> Option<String> {
    let rest = line.trim_start_matches('#').trim();
    if rest.is_empty() { None } else { Some(rest.to_string()) }
}

// ── Per-request segment parsing ──────────────────────────────────────────────

/// Parse the lines of one request segment (everything after its `###` line).
/// Returns any variable declarations found alongside the parsed `Request`.
#[allow(clippy::type_complexity)]
fn parse_request_segment(
    lines: &[(usize, &str)],
    name: Option<String>,
) -> Result<(Vec<Variable>, Option<(Request, RequestMeta)>), ParseError> {
    let mut variables: Vec<Variable> = Vec::new();
    let mut i = 0;

    // Consume leading comments, blanks, and variable declarations.
    while i < lines.len() {
        let (line_num, line) = lines[i];
        if line.trim().is_empty() || is_comment(line) {
            i += 1;
            continue;
        }
        if line.trim_start().starts_with('@') {
            if let Some(var) = parse_variable_line(line, line_num)? {
                variables.push(var);
            }
            i += 1;
            continue;
        }
        break;
    }

    if i >= lines.len() {
        return Ok((variables, None));
    }

    let (req_line_num, req_line) = lines[i];
    let (method, url, version) = parse_request_line(req_line, req_line_num)?;
    i += 1;

    // Parse headers until a blank line or end of segment.
    let mut headers: Vec<(String, String)> = Vec::new();
    let mut header_lines: Vec<usize> = Vec::new();
    while i < lines.len() {
        let (line_num, line) = lines[i];
        if line.trim().is_empty() {
            i += 1;
            break;
        }
        if is_comment(line) {
            i += 1;
            continue;
        }
        if line.trim_start().starts_with('@') {
            if let Some(var) = parse_variable_line(line, line_num)? {
                variables.push(var);
            }
            i += 1;
            continue;
        }
        headers.push(parse_header_line(line, line_num)?);
        header_lines.push(line_num);
        i += 1;
    }

    // Capture the body verbatim; only trim trailing newlines.
    let body_line = lines.get(i).map(|(ln, _)| *ln).unwrap_or(0);
    let body = if i < lines.len() {
        let body_str = lines[i..]
            .iter()
            .map(|(_, l)| *l)
            .collect::<Vec<_>>()
            .join("\n");
        let body_str = body_str.trim_end_matches('\n').to_string();
        if body_str.is_empty() { None } else { Some(body_str) }
    } else {
        None
    };

    let meta = RequestMeta { url_line: req_line_num, header_lines, body_line };
    let request = Request { name, method, url, headers, version, body };
    Ok((variables, Some((request, meta))))
}

// ── Sub-line parsers (winnow-backed) ────────────────────────────────────────

/// Parse a request line of the form `METHOD URL [HTTP_VERSION]`.
/// Uses `winnow_parse_request_inner` for the actual parsing and discriminates
/// the error kind to produce the correct `ParseError` variant.
fn parse_request_line(
    line: &str,
    line_num: usize,
) -> Result<(HttpMethod, String, HttpVersion), ParseError> {
    // Capture the method token before winnow touches the input, so we can
    // produce a useful error message if the method is unrecognised.
    let method_tok = line.split_whitespace().next().unwrap_or("").to_string();

    let mut input = line.trim();
    winnow_parse_request_inner(&mut input).map_err(|_| {
        let known_method = matches!(
            method_tok.as_str(),
            "GET" | "POST" | "PUT" | "DELETE" | "PATCH"
                | "OPTIONS" | "HEAD" | "CONNECT" | "TRACE"
        );
        if known_method {
            ParseError::InvalidRequestLine { line: line_num, content: line.to_string() }
        } else {
            ParseError::UnknownMethod { method: method_tok, line: line_num }
        }
    })
}

/// Parse a header line of the form `Key: Value`.
fn parse_header_line(line: &str, line_num: usize) -> Result<(String, String), ParseError> {
    let mut input = line;
    winnow_parse_header(&mut input).map_err(|_| ParseError::InvalidHeader {
        line: line_num,
        content: line.to_string(),
    })
}

/// Parse an `@name = value` variable declaration line.
/// Returns `Ok(None)` for blank or comment lines.
fn parse_variable_line(line: &str, line_num: usize) -> Result<Option<Variable>, ParseError> {
    let trimmed = line.trim();
    if trimmed.is_empty() || is_comment(line) {
        return Ok(None);
    }
    if !trimmed.starts_with('@') {
        return Ok(None);
    }
    let mut input = trimmed;
    winnow_parse_variable(&mut input).map_err(|_| ParseError::InvalidVariableDeclaration {
        line: line_num,
        content: line.to_string(),
    })
    .map(Some)
}

// ── Winnow inner parsers ─────────────────────────────────────────────────────
// These return `winnow::Result` and keep all error types consistent so the
// compiler can infer them. Error conversion to `ParseError` happens in the
// adapter functions above.

fn winnow_parse_request_inner(input: &mut &str) -> WResult<(HttpMethod, String, HttpVersion)> {
    let method = winnow_method(input)?;
    let url = take_while(1.., |c: char| !c.is_whitespace())
        .parse_next(input)?
        .to_string();
    let version = opt(preceded(space1, winnow_version))
        .parse_next(input)?
        .unwrap_or(HttpVersion::Http11);
    Ok((method, url, version))
}

fn winnow_parse_header(input: &mut &str) -> WResult<(String, String)> {
    let key = take_until(0.., ":").parse_next(input)?;
    literal(":").parse_next(input)?;
    let _ = space0.parse_next(input)?;
    let value = take_while(0.., |_: char| true).parse_next(input)?;
    Ok((key.trim().to_string(), value.trim().to_string()))
}

fn winnow_parse_variable(input: &mut &str) -> WResult<Variable> {
    literal("@").parse_next(input)?;
    let name = take_until(0.., "=").parse_next(input)?;
    literal("=").parse_next(input)?;
    let _ = space0.parse_next(input)?;
    let value = take_while(0.., |_: char| true).parse_next(input)?;
    Ok(Variable {
        name: name.trim().to_string(),
        value: value.trim().to_string(),
    })
}

fn winnow_method(input: &mut &str) -> WResult<HttpMethod> {
    terminated(
        alt((
            literal("DELETE").map(|_| HttpMethod::Delete),
            literal("PATCH").map(|_| HttpMethod::Patch),
            literal("HEAD").map(|_| HttpMethod::Head),
            literal("OPTIONS").map(|_| HttpMethod::Options),
            literal("GET").map(|_| HttpMethod::Get),
            literal("POST").map(|_| HttpMethod::Post),
            literal("PUT").map(|_| HttpMethod::Put),
            literal("CONNECT").map(|_| HttpMethod::Connect),
            literal("TRACE").map(|_| HttpMethod::Trace),
        )),
        space1,
    )
    .parse_next(input)
}

fn winnow_version(input: &mut &str) -> WResult<HttpVersion> {
    alt((
        literal("HTTP/2.0").map(|_| HttpVersion::Http20),
        literal("HTTP/1.1").map(|_| HttpVersion::Http11),
        literal("HTTP/1.0").map(|_| HttpVersion::Http10),
    ))
    .parse_next(input)
}

// ── Utilities ────────────────────────────────────────────────────────────────

/// Returns `true` if `line` is a `//` or `#` comment (but NOT a `###` separator).
fn is_comment(line: &str) -> bool {
    let t = line.trim_start();
    (t.starts_with("//") || t.starts_with('#')) && !t.starts_with("###")
}

// ── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_comment() {
        assert!(is_comment("// this is a comment"));
        assert!(is_comment("# this is a comment"));
        assert!(!is_comment("### this is a separator"));
        assert!(!is_comment("GET /foo"));
        assert!(!is_comment(""));
    }

    #[test]
    fn test_parse_separator_name_with_name() {
        assert_eq!(
            parse_separator_name("### My Request"),
            Some("My Request".to_string())
        );
    }

    #[test]
    fn test_parse_separator_name_bare() {
        assert_eq!(parse_separator_name("###"), None);
        assert_eq!(parse_separator_name("###   "), None);
    }

    #[test]
    fn test_parse_request_line_no_version() {
        let (method, url, version) = parse_request_line("GET /api/v1/resource", 1).unwrap();
        assert_eq!(method, HttpMethod::Get);
        assert_eq!(url, "/api/v1/resource");
        assert_eq!(version, HttpVersion::Http11);
    }

    #[test]
    fn test_parse_request_line_with_version() {
        let (method, url, version) =
            parse_request_line("GET /api/v1/resource HTTP/2.0", 1).unwrap();
        assert_eq!(method, HttpMethod::Get);
        assert_eq!(url, "/api/v1/resource");
        assert_eq!(version, HttpVersion::Http20);
    }

    #[test]
    fn test_parse_request_line_unknown_method() {
        let err = parse_request_line("FETCH /api/v1/resource", 5).unwrap_err();
        assert_eq!(
            err,
            ParseError::UnknownMethod { method: "FETCH".to_string(), line: 5 }
        );
    }

    #[test]
    fn test_parse_header_line_valid() {
        let result = parse_header_line("Content-Type: application/json", 1).unwrap();
        assert_eq!(
            result,
            ("Content-Type".to_string(), "application/json".to_string())
        );
    }

    #[test]
    fn test_parse_header_line_value_with_colon() {
        // Values containing colons (e.g. URLs) must be captured in full.
        let result = parse_header_line("Location: http://example.com/path", 1).unwrap();
        assert_eq!(
            result,
            ("Location".to_string(), "http://example.com/path".to_string())
        );
    }

    #[test]
    fn test_parse_header_line_no_colon() {
        let err = parse_header_line("ContentType application/json", 3).unwrap_err();
        assert_eq!(
            err,
            ParseError::InvalidHeader {
                line: 3,
                content: "ContentType application/json".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_variable_line_valid() {
        let result = parse_variable_line("@baseUrl = https://api.example.com", 1).unwrap();
        assert_eq!(
            result,
            Some(Variable {
                name: "baseUrl".to_string(),
                value: "https://api.example.com".to_string(),
            })
        );
    }

    #[test]
    fn test_parse_variable_line_value_with_spaces() {
        let result = parse_variable_line("@greeting = Hello World", 1).unwrap();
        assert_eq!(
            result,
            Some(Variable {
                name: "greeting".to_string(),
                value: "Hello World".to_string(),
            })
        );
    }

    #[test]
    fn test_parse_variable_line_value_with_equals() {
        // Only the first `=` is the separator; the rest belongs to the value.
        let result = parse_variable_line("@query = a=1&b=2", 1).unwrap();
        assert_eq!(
            result,
            Some(Variable {
                name: "query".to_string(),
                value: "a=1&b=2".to_string(),
            })
        );
    }

    #[test]
    fn test_parse_variable_line_malformed() {
        let err = parse_variable_line("@missingEquals", 2).unwrap_err();
        assert_eq!(
            err,
            ParseError::InvalidVariableDeclaration {
                line: 2,
                content: "@missingEquals".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_variable_line_blank_and_comment() {
        assert_eq!(parse_variable_line("", 1).unwrap(), None);
        assert_eq!(parse_variable_line("// comment", 1).unwrap(), None);
        assert_eq!(parse_variable_line("# comment", 1).unwrap(), None);
    }
}
