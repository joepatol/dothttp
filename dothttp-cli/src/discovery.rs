use std::path::Path;

use dothttp_runner::RunnerRequest;
use walkdir::WalkDir;

pub struct DiscoveredRequest {
    pub label: String,
    pub request: RunnerRequest,
}

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

        let content = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("warning: could not read {}: {e}", relative.display());
                continue;
            }
        };

        let http_file = match dothttp_parser::parse_http_file(&content) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("warning: could not parse {}: {e}", relative.display());
                continue;
            }
        };

        for request in http_file.requests.into_iter() {
            let name_part = request
                .name
                .clone()
                .unwrap_or_else(|| format!("{} {}", request.method.to_string(), request.url));
            let label = format!("{}::{}", relative.display(), name_part);
            discovered.push(DiscoveredRequest {
                label,
                request: RunnerRequest::from(request),
            });
        }
    }

    discovered
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
    fn test_malformed_file_is_skipped() {
        let dir = TempDir::new().unwrap();
        temp_http_file(&dir, "bad.http", "### Bad\nFETCH http://example.com\n");
        temp_http_file(&dir, "good.http", "### OK\nGET http://example.com\n");

        let results = discover(dir.path());
        assert_eq!(results.len(), 1);
        assert!(results[0].label.contains("good.http"));
    }
}
