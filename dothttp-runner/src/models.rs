use crate::error::RunnerError;

#[derive(Debug, Clone, PartialEq)]
pub struct RunnerRequest {
    pub name: Option<String>,
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

#[derive(Debug)]
pub struct RequestResult {
    pub request: RunnerRequest,
    pub response: Result<HttpResponse, RunnerError>,
}
