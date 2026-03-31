use std::path::Path;

use dothttp_runner::RunnerRequest;
use walkdir::WalkDir;

pub struct DiscoveredRequest {
    pub label: String,
    /// The short identifier: request name for named requests, "METHOD URL" for unnamed.
    pub identifier: String,
    pub request: RunnerRequest,
}

/// Discover all .http requests under a directory (recursive).
pub fn discover(dir: &Path) -> Vec<DiscoveredRequest> {
    let mut discovered = Vec::new();

    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("http"))
    {
        let path = entry.path();
        let relative = path.strip_prefix(dir).unwrap_or(path);
        collect_from_file(path, relative, &mut discovered);
    }

    discovered
}

/// Discover all requests in a single .http file.
pub fn discover_file(path: &Path) -> Vec<DiscoveredRequest> {
    let mut discovered = Vec::new();
    collect_from_file(path, path, &mut discovered);
    discovered
}

fn collect_from_file(path: &Path, label_prefix: &Path, out: &mut Vec<DiscoveredRequest>) {
    let content = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("warning: could not read {}: {e}", path.display());
            return;
        }
    };

    let http_file = match dothttp_parser::parse_http_file(&content) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("warning: could not parse {}: {e}", path.display());
            return;
        }
    };

    for request in http_file.requests.into_iter() {
        let identifier = request
            .name
            .clone()
            .unwrap_or_else(|| format!("{} {}", request.method.to_string(), request.url));
        let label = format!("{}::{}", label_prefix.display(), identifier);
        out.push(DiscoveredRequest {
            label,
            identifier,
            request: RunnerRequest::from(request),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    fn temp_http_file(dir: &TempDir, name: &str, content: &str) {
        let path = dir.path().join(name);
        let mut f = fs::File::create(path).unwrap();
        f.write_all(content.as_bytes()).unwrap();
    }

    #[test]
    fn test_named_and_unnamed_labels() {
        let dir = TempDir::new().unwrap();
        temp_http_file(
            &dir,
            "api.http",
            "### Get Users\nGET http://example.com/users\n\n###\nGET http://example.com/posts\n",
        );

        let results = discover(dir.path());
        assert_eq!(results.len(), 2);
        assert!(results[0].label.ends_with("::Get Users"), "label: {}", results[0].label);
        assert!(results[1].label.ends_with("::GET http://example.com/posts"), "label: {}", results[1].label);
    }

    #[test]
    fn test_named_identifier() {
        let dir = TempDir::new().unwrap();
        temp_http_file(&dir, "api.http", "### Get Users\nGET http://example.com/users\n");

        let results = discover(dir.path());
        assert_eq!(results[0].identifier, "Get Users");
    }

    #[test]
    fn test_unnamed_identifier_is_method_url() {
        let dir = TempDir::new().unwrap();
        temp_http_file(&dir, "api.http", "###\nGET http://example.com/posts\n");

        let results = discover(dir.path());
        assert_eq!(results[0].identifier, "GET http://example.com/posts");
    }

    #[test]
    fn test_malformed_file_is_skipped() {
        let dir = TempDir::new().unwrap();
        temp_http_file(&dir, "bad.http", "### Bad\nFETCH http://example.com\n");
        temp_http_file(&dir, "good.http", "### OK\nGET http://example.com\n");

        let results = discover(dir.path());
        assert_eq!(results.len(), 1);
        assert!(results[0].label.contains("good.http"));
    }

    #[test]
    fn test_discover_file_single_file() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("api.http");
        fs::write(&path, "### Get Users\nGET http://example.com/users\n").unwrap();

        let results = discover_file(&path);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].identifier, "Get Users");
    }
}
