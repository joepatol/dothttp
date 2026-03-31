use crate::models::RunnerRequest;
use dothttp_parser::Request;

impl From<Request> for RunnerRequest {
    fn from(r: Request) -> Self {
        RunnerRequest {
            name: r.name,
            method: r.method.to_string(),
            url: r.url,
            headers: r.headers,
            body: r.body,
        }
    }
}
