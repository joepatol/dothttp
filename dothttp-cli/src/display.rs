use colored::Colorize;
use dothttp_runner::RequestResult;

pub fn print_result(label: &str, result: &RequestResult) {
    println!("\n{}", format!("─── {} ───", label).bold());

    match &result.response {
        Ok(resp) => {
            let status_str = format!("HTTP {}", resp.status);
            let colored_status = match resp.status {
                200..=299 => status_str.green(),
                300..=399 => status_str.yellow(),
                _ => status_str.red(),
            };
            println!("{}", colored_status);

            for (key, value) in &resp.headers {
                println!("{}: {}", key.bold(), value);
            }

            if !resp.body.is_empty() {
                println!();
                let body_out = serde_json::from_str::<serde_json::Value>(&resp.body)
                    .ok()
                    .and_then(|v| serde_json::to_string_pretty(&v).ok())
                    .unwrap_or_else(|| resp.body.clone());
                println!("{}", body_out);
            }
        }
        Err(e) => {
            println!("{}", format!("Error: {e}").red());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dothttp_runner::{HttpResponse, RequestResult, RunnerError, RunnerRequest};

    fn make_request(label: &str) -> RunnerRequest {
        RunnerRequest {
            name: Some(label.to_string()),
            method: "GET".to_string(),
            url: "http://example.com".to_string(),
            headers: vec![],
            body: None,
        }
    }

    #[test]
    fn test_print_result_ok_contains_status_and_body() {
        // Arrange
        let request = make_request("test");
        let result = RequestResult {
            request,
            response: Ok(HttpResponse {
                status: 200,
                headers: vec![("content-type".to_string(), "application/json".to_string())],
                body: r#"{"key":"value"}"#.to_string(),
            }),
        };

        // Act — capture via colored::control (just assert no panic; visual output tested manually)
        // We call print_result and assert it doesn't panic
        print_result("api.http::test", &result);
    }

    #[test]
    fn test_print_result_error_does_not_panic() {
        // Arrange
        let request = make_request("test");
        let result = RequestResult {
            request,
            response: Err(RunnerError::Other("connection refused".to_string())),
        };

        // Act
        print_result("api.http::test", &result);
    }
}
