mod error;
mod interpolation;
mod models;
mod parser;

pub use error::ParseError;
pub use models::{HttpFile, HttpMethod, HttpVersion, Request, Variable};
pub use parser::parse_http_file;

#[cfg(test)]
mod httpfile_tests {
    use std::fs;
    use std::path::PathBuf;

    use crate::models::{HttpMethod, HttpVersion};

    use super::*;

    fn fixture_path(filename: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("httpfiles")
            .join(filename)
    }

    fn read_fixture(filename: &str) -> String {
        fs::read_to_string(fixture_path(filename))
            .unwrap_or_else(|_| panic!("Failed to read fixture '{filename}'"))
    }

    // ── Existing fixture tests (updated to parse_http_file) ─────────────────

    #[test]
    fn test_basic_get_request() {
        let data = read_fixture("getrequest.http");
        let file = parse_http_file(&data).expect("Failed to parse HTTP file");

        assert_eq!(file.requests.len(), 1);
        let req = &file.requests[0];
        assert_eq!(req.name, Some("Basic GET request".to_string()));
        assert_eq!(req.method, HttpMethod::Get);
        assert_eq!(req.url, "http://api.example.com/users/123");
        assert_eq!(req.headers.len(), 2);
        assert_eq!(req.headers[0], ("Accept".to_string(), "application/json".to_string()));
        assert_eq!(req.headers[1], ("Origin".to_string(), "http://somewhere.com".to_string()));
        assert_eq!(req.version, HttpVersion::Http11);
        assert_eq!(req.body, None);
    }

    #[test]
    fn test_no_named_basic_get() {
        let data = read_fixture("nonamedget.http");
        let file = parse_http_file(&data).expect("Failed to parse HTTP file");
        
        assert_eq!(file.requests.len(), 1);
        let req = &file.requests[0];
        assert_eq!(req.name, None);
        assert_eq!(req.method, HttpMethod::Get);
        assert_eq!(req.url, "http://api.example.com/users/123");
    }

    #[test]
    fn test_get_request_with_http_version() {
        let data = read_fixture("httpversion.http");
        let file = parse_http_file(&data).expect("Failed to parse HTTP file");

        assert_eq!(file.requests.len(), 1);
        let req = &file.requests[0];
        assert_eq!(req.name, Some("GET with http version".to_string()));
        assert_eq!(req.method, HttpMethod::Get);
        assert_eq!(req.url, "http://api.example.com/users/123");
        assert_eq!(req.version, HttpVersion::Http20);
    }

    #[test]
    fn test_post_request_body_verbatim() {
        let data = read_fixture("postrequest.http");
        let file = parse_http_file(&data).expect("Failed to parse HTTP file");

        assert_eq!(file.requests.len(), 1);
        let req = &file.requests[0];
        assert_eq!(req.name, Some("Basic POST request".to_string()));
        assert_eq!(req.method, HttpMethod::Post);
        assert_eq!(req.url, "http://api.example.com/users/123");
        assert_eq!(req.headers, vec![("Content-Type".to_string(), "application/json".to_string())]);
        assert_eq!(req.version, HttpVersion::Http11);

        // Body must be preserved verbatim – no whitespace stripping
        let body = req.body.as_deref().expect("Expected a body");
        assert!(body.contains("    \"name\": \"John Doe\""), "body whitespace must be preserved");
        assert!(body.contains('\n'), "body newlines must be preserved");
    }

    // ── Multi-request ────────────────────────────────────────────────────────

    #[test]
    fn test_multi_request_count_and_order() {
        let data = read_fixture("multirequest.http");
        let file = parse_http_file(&data).expect("Failed to parse HTTP file");

        assert_eq!(file.requests.len(), 2);
        assert_eq!(file.requests[0].method, HttpMethod::Get);
        assert_eq!(file.requests[1].method, HttpMethod::Post);
    }

    #[test]
    fn test_named_and_unnamed_requests() {
        let input = "### Named\nGET http://example.com\n\n###\nGET http://example.com/two\n";
        let file = parse_http_file(input).unwrap();
        
        assert_eq!(file.requests[0].name, Some("Named".to_string()));
        assert_eq!(file.requests[1].name, None);
    }

    // ── Variables ────────────────────────────────────────────────────────────

    #[test]
    fn test_variable_declarations_collected() {
        let data = read_fixture("variables.http");
        let file = parse_http_file(&data).expect("Failed to parse HTTP file");
        dbg!("{:#?}", &file);
        assert!(
            file.variables.iter().any(|v| v.name == "baseUrl"),
            "expected 'baseUrl' variable"
        );
        assert!(
            file.variables.iter().any(|v| v.name == "token"),
            "expected 'token' variable"
        );
    }

    // ── Comments ─────────────────────────────────────────────────────────────

    #[test]
    fn test_comment_lines_ignored() {
        let input = "// file-level comment\n# another comment\n### My Request\n// inline comment\nGET http://example.com\n";
        let file = parse_http_file(input).expect("Failed to parse with comments");

        assert_eq!(file.requests.len(), 1);
        assert_eq!(file.requests[0].method, HttpMethod::Get);
    }

    // ── Body preservation ────────────────────────────────────────────────────

    #[test]
    fn test_json_body_verbatim() {
        let input = "### Post\nPOST http://example.com\nContent-Type: application/json\n\n{\n    \"key\": \"value\"\n}\n";
        let file = parse_http_file(input).unwrap();

        let body = file.requests[0].body.as_deref().expect("expected body");
        assert_eq!(body, "{\n    \"key\": \"value\"\n}");
    }

    #[test]
    fn test_empty_file() {
        let file = parse_http_file("").unwrap();
        assert!(file.requests.is_empty());
        assert!(file.variables.is_empty());
    }

    // ── Error cases ──────────────────────────────────────────────────────────

    #[test]
    fn test_unknown_method_error() {
        let input = "### Bad\nFETCH http://example.com\n";
        let err = parse_http_file(input).unwrap_err();
        assert!(
            matches!(err, ParseError::UnknownMethod { ref method, line: _ } if method == "FETCH"),
            "expected UnknownMethod, got: {err}"
        );
    }

    #[test]
    fn test_invalid_header_error() {
        let input = "### Bad\nGET http://example.com\nContentType application/json\n";
        let err = parse_http_file(input).unwrap_err();
        assert!(
            matches!(err, ParseError::InvalidHeader { .. }),
            "expected InvalidHeader, got: {err}"
        );
    }

    #[test]
    fn test_parse_error_display_includes_line_number() {
        let err = ParseError::UnknownMethod {
            method: "FETCH".to_string(),
            line: 5,
        };
        let msg = err.to_string();
        assert!(msg.contains("line 5"), "expected line number in: {msg}");
        assert!(msg.contains("FETCH"), "expected method name in: {msg}");
    }

    #[test]
    fn test_parse_error_is_std_error() {
        let err: Box<dyn std::error::Error> =
            Box::new(ParseError::UnexpectedEndOfInput);
        assert_eq!(err.to_string(), "Parse error: unexpected end of input");
    }

    // ── Variable interpolation ───────────────────────────────────────────────

    #[test]
    fn test_url_interpolation() {
        let input = "@baseUrl = https://api.example.com\n### R\nGET {{baseUrl}}/users\n";
        let file = parse_http_file(input).unwrap();
        assert_eq!(file.requests[0].url, "https://api.example.com/users");
    }

    #[test]
    fn test_header_value_interpolation() {
        let input = "@token = abc123\n### R\nGET http://example.com\nAuthorization: Bearer {{token}}\n";
        let file = parse_http_file(input).unwrap();
        assert_eq!(file.requests[0].headers[0].1, "Bearer abc123");
    }

    #[test]
    fn test_body_interpolation() {
        let input = "@userId = 42\n### R\nPOST http://example.com\nContent-Type: application/json\n\n{\"id\": {{userId}}}\n";
        let file = parse_http_file(input).unwrap();
        assert_eq!(file.requests[0].body.as_deref().unwrap(), "{\"id\": 42}");
    }

    #[test]
    fn test_multiple_references_in_url() {
        let input = "@scheme = https\n@host = api.example.com\n### R\nGET {{scheme}}://{{host}}/path\n";
        let file = parse_http_file(input).unwrap();
        assert_eq!(file.requests[0].url, "https://api.example.com/path");
    }

    #[test]
    fn test_variable_declared_after_usage() {
        // Variable declared in a later segment must still resolve references in earlier segments,
        // because all variables are collected before interpolation begins.
        let input = "### First\nGET {{baseUrl}}/users\n\n### Second\n@baseUrl = https://api.example.com\nGET http://other.com\n";
        let file = parse_http_file(input).unwrap();
        assert_eq!(file.requests[0].url, "https://api.example.com/users");
    }

    #[test]
    fn test_undefined_variable_error() {
        let input = "### R\nGET {{missing}}/users\n";
        let err = parse_http_file(input).unwrap_err();
        assert!(
            matches!(err, ParseError::UndefinedVariable { ref name, .. } if name == "missing"),
            "expected UndefinedVariable, got: {err}"
        );
    }

    #[test]
    fn test_no_references_unchanged() {
        let input = "@baseUrl = https://api.example.com\n### R\nGET http://example.com\n";
        let file = parse_http_file(input).unwrap();
        assert_eq!(file.requests[0].url, "http://example.com");
    }

    #[test]
    fn test_header_key_not_interpolated() {
        let input = "@name = Authorization\n### R\nGET http://example.com\n{{name}}: Bearer token\n";
        let file = parse_http_file(input).unwrap();
        // The key should be the literal string, not substituted.
        assert_eq!(file.requests[0].headers[0].0, "{{name}}");
    }

    #[test]
    fn test_undefined_variable_display() {
        let err = ParseError::UndefinedVariable { name: "token".to_string(), line: 5 };
        let msg = err.to_string();
        assert!(msg.contains("token"), "expected variable name in: {msg}");
        assert!(msg.contains("line 5"), "expected line number in: {msg}");
    }

    #[test]
    fn test_interpolation_fixture() {
        let data = read_fixture("interpolation.http");
        let file = parse_http_file(&data).unwrap();

        assert_eq!(file.requests.len(), 2);
        assert_eq!(file.requests[0].url, "https://api.example.com/users/42");
        assert_eq!(file.requests[0].headers[0].1, "Bearer secret-token-123");
        assert!(file.requests[1].body.as_deref().unwrap().contains("42"));
    }
}
