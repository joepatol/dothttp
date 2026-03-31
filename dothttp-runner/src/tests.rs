use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::models::RunnerRequest;
use crate::runner::run;

fn get_request(url: &str) -> RunnerRequest {
    RunnerRequest {
        name: None,
        method: "GET".to_string(),
        url: url.to_string(),
        headers: vec![],
        body: None,
    }
}

// Task 5.2 — empty list returns empty result
#[tokio::test]
async fn test_empty_list_returns_empty_results() {
    let results = run(vec![]).await;
    assert!(results.is_empty());
}

// Task 5.3 — single GET returns correct status, headers, body
#[tokio::test]
async fn test_single_get_request() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/hello"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("hello world")
                .insert_header("x-custom", "yes"),
        )
        .mount(&server)
        .await;

    let url = format!("{}/hello", server.uri());
    let results = run(vec![get_request(&url)]).await;

    assert_eq!(results.len(), 1);
    let resp = results[0].response.as_ref().expect("expected Ok response");
    assert_eq!(resp.status, 200);
    assert_eq!(resp.body, "hello world");
    assert!(resp.headers.iter().any(|(k, v)| k == "x-custom" && v == "yes"));
}

// Task 5.4 — multiple requests, results paired in input order
#[tokio::test]
async fn test_multiple_requests_result_order_matches_input() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/one"))
        .respond_with(ResponseTemplate::new(200).set_body_string("one"))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/two"))
        .respond_with(ResponseTemplate::new(200).set_body_string("two"))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/three"))
        .respond_with(ResponseTemplate::new(200).set_body_string("three"))
        .mount(&server)
        .await;

    let requests = vec![
        get_request(&format!("{}/one", server.uri())),
        get_request(&format!("{}/two", server.uri())),
        get_request(&format!("{}/three", server.uri())),
    ];
    let input_urls: Vec<String> = requests.iter().map(|r| r.url.clone()).collect();

    let results = run(requests).await;

    assert_eq!(results.len(), 3);
    for (i, result) in results.iter().enumerate() {
        assert_eq!(result.request.url, input_urls[i]);
    }
    assert_eq!(results[0].response.as_ref().unwrap().body, "one");
    assert_eq!(results[1].response.as_ref().unwrap().body, "two");
    assert_eq!(results[2].response.as_ref().unwrap().body, "three");
}

// Task 5.5 — one failing request produces Err, others succeed
#[tokio::test]
async fn test_failing_request_does_not_affect_others() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/ok"))
        .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
        .mount(&server)
        .await;

    let bad_url = "http://127.0.0.1:1"; // nothing listening here
    let requests = vec![
        get_request(&format!("{}/ok", server.uri())),
        get_request(bad_url),
        get_request(&format!("{}/ok", server.uri())),
    ];

    let results = run(requests).await;

    assert_eq!(results.len(), 3);
    assert!(results[0].response.is_ok(), "first request should succeed");
    assert!(results[1].response.is_err(), "second request should fail");
    assert!(results[2].response.is_ok(), "third request should succeed");
}

// Task 5.6 — From<dothttp_parser::Request> conversion preserves all fields
#[test]
fn test_from_parser_request_preserves_fields() {
    use dothttp_parser::parse_http_file;

    let input = "### My Request\nPOST http://example.com/api\nContent-Type: application/json\n\n{\"key\":\"value\"}\n";
    let file = parse_http_file(input).expect("parse failed");
    let parsed = file.requests.into_iter().next().expect("no requests");

    let runner_req = RunnerRequest::from(parsed);

    assert_eq!(runner_req.name, Some("My Request".to_string()));
    assert_eq!(runner_req.method, "POST");
    assert_eq!(runner_req.url, "http://example.com/api");
    assert_eq!(runner_req.headers, vec![("Content-Type".to_string(), "application/json".to_string())]);
    assert_eq!(runner_req.body.as_deref(), Some("{\"key\":\"value\"}"));
}

// Task 5.7 — non-2xx responses are Ok(HttpResponse), not Err
#[tokio::test]
async fn test_non_2xx_responses_are_ok() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/not-found"))
        .respond_with(ResponseTemplate::new(404).set_body_string("not found"))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/error"))
        .respond_with(ResponseTemplate::new(500).set_body_string("server error"))
        .mount(&server)
        .await;

    let results = run(vec![
        get_request(&format!("{}/not-found", server.uri())),
        get_request(&format!("{}/error", server.uri())),
    ])
    .await;

    let resp_404 = results[0].response.as_ref().expect("expected Ok for 404");
    assert_eq!(resp_404.status, 404);

    let resp_500 = results[1].response.as_ref().expect("expected Ok for 500");
    assert_eq!(resp_500.status, 500);
}
