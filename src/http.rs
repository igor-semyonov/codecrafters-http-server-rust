use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Response {
    pub version: HttpVersion,
    pub code: ResponseCode,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Debug, Copy, Clone)]
pub enum ResponseCode {
    C200,
    C201,
    C404,
    C409,
}
impl From<ResponseCode> for u32 {
    fn from(code: ResponseCode) -> Self {
        match code {
            ResponseCode::C200 => 200,
            ResponseCode::C201 => 201,
            ResponseCode::C404 => 404,
            ResponseCode::C409 => 409,
        }
    }
}
impl From<ResponseCode> for &str {
    fn from(code: ResponseCode) -> Self {
        match code {
            ResponseCode::C200 => "OK",
            ResponseCode::C201 => "Created",
            ResponseCode::C404 => "Not Found",
            ResponseCode::C409 => "Conflit",
        }
    }
}

impl From<Response> for String {
    fn from(response: Response) -> Self {
        let status_code: u32 = response
            .code
            .into();
        let status_text: &str = response
            .code
            .into();
        let headers = response
            .headers
            .iter()
            .map(
                |(k, v)| {
                    format!(
                        "{}: {}",
                        k, v
                    )
                },
            )
            .collect::<Vec<String>>()
            .join("\r\n");

        format!(
            "HTTP/{} {} {}\r\n{}\r\n\r\n{}",
            std::convert::Into::<&str>::into(
                response.version
            ),
            status_code,
            status_text,
            headers,
            response.body,
        )
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Request {
    request: String,
    pub method: HttpMethod,
    pub target: String,
    pub version: HttpVersion,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Put,
    Post,
}
#[derive(Debug, Copy, Clone)]
pub enum HttpVersion {
    V1_1,
}
impl From<HttpVersion> for &str {
    fn from(version: HttpVersion) -> Self {
        match version {
            HttpVersion::V1_1 => "1.1",
        }
    }
}

impl From<&[u8]> for Request {
    fn from(bytes: &[u8]) -> Self {
        let request = String::from_utf8(bytes.to_vec())
            .expect(
                "Could not convert requset to UTF8 string.",
            );
        let (request_line, request_remaining) = request
            .split_once("\r\n")
            .unwrap_or((
                "GET / HTTP/1.1",
                "",
            ));
        let mut request_line =
            request_line.split_ascii_whitespace();
        let method = match request_line
            .next()
            .unwrap_or("Get")
        {
            "GET" => HttpMethod::Get,
            "PUT" => HttpMethod::Put,
            "POST" => HttpMethod::Post,
            _ => HttpMethod::Get,
        };
        let target = request_line
            .next()
            .unwrap_or("/")
            .to_string();
        let version = match request_line
            .next()
            .unwrap_or("HTTP/1.1")
        {
            "HTTP/1.1" => HttpVersion::V1_1,
            _ => HttpVersion::V1_1,
        };
        let (headers, body) = request_remaining
            .split_once("\r\n\r\n")
            .unwrap_or((
                request_remaining,
                "",
            ));
        let not_header = (
            "Not".to_string(),
            "Header".to_string(),
        );
        let headers = headers
            .split("\r\n")
            .map(
                |header| -> (
                    String,
                    String,
                ) {
                    match header.split_once(":") {
                        Some(v) => {
                            let (
                                header_key,
                                mut header_value,
                            ) = v;
                            header_value =
                                header_value.trim_start();
                            (
                                header_key.to_string(),
                                header_value.to_string(),
                            )
                        }
                        None => not_header.clone(),
                    }
                },
            )
            .filter(|h| *h != not_header)
            .collect::<HashMap<_, _>>();

        let body = body.to_string();

        Request {
            request,
            method,
            target,
            version,
            headers,
            body,
        }
    }
}
