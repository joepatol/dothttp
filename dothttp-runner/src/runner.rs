use futures::future::join_all;
use reqwest::Client;

use crate::error::RunnerError;
use crate::models::{HttpResponse, RequestResult, RunnerRequest};

pub async fn run(requests: Vec<RunnerRequest>) -> Vec<RequestResult> {
    let client = Client::new();
    let futures = requests
        .into_iter()
        .map(|req| execute_request(&client, req));
    join_all(futures).await
}

async fn execute_request(client: &Client, request: RunnerRequest) -> RequestResult {
    let response = send(client, &request).await;
    RequestResult { request, response }
}

async fn send(client: &Client, request: &RunnerRequest) -> Result<HttpResponse, RunnerError> {
    let method = reqwest::Method::from_bytes(request.method.as_bytes())
        .map_err(|e| RunnerError::Other(format!("Invalid HTTP method '{}': {e}", request.method)))?;

    let mut builder = client.request(method, &request.url);

    for (key, value) in &request.headers {
        builder = builder.header(key.as_str(), value.as_str());
    }

    if let Some(body) = &request.body {
        builder = builder.body(body.clone());
    }

    let resp = builder.send().await?;

    let status = resp.status().as_u16();
    let headers = resp
        .headers()
        .iter()
        .filter_map(|(k, v)| {
            v.to_str()
                .ok()
                .map(|v_str| (k.to_string(), v_str.to_string()))
        })
        .collect();
    let body = resp.text().await?;

    Ok(HttpResponse {
        status,
        headers,
        body,
    })
}
