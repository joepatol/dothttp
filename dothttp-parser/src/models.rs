#[derive(Debug, PartialEq)]
pub enum HttpVersion {
    Http10,
    Http11,
    Http20,
}

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Options,
    Head,
    Connect,
    Trace,
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Options => "OPTIONS",
            HttpMethod::Head => "HEAD",
            HttpMethod::Connect => "CONNECT",
            HttpMethod::Trace => "TRACE",
        };
        f.write_str(s)
    }
}

#[derive(Debug, PartialEq)]
pub struct Request {
    pub name: Option<String>,
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub version: HttpVersion,
    pub body: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct Variable {
    pub name: String,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct HttpFile {
    pub variables: Vec<Variable>,
    pub requests: Vec<Request>,
}
