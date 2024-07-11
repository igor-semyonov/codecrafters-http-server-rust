use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    request: String,
    request_line: RequestLine,
    headers: HashMap<String, String>,
    body: String,
}

#[derive(Debug)]
struct RequestLine {
    method: HttpMethod,
    target: String,
    version: HttpVersion,
}
#[derive(Debug)]
enum HttpMethod {
    Get,
    Put,
    Post,
}
#[derive(Debug)]
enum HttpVersion {
    V1_1,
}

impl From<&str> for RequestLine {
    fn from(s: &str) -> Self {
        let mut x = s.split_ascii_whitespace();
        let method = match x
            .next()
            .unwrap()
        {
            "Get" => HttpMethod::Get,
            "Put" => HttpMethod::Put,
            "Post" => HttpMethod::Post,
            _ => HttpMethod::Get,
        };
        let target = x
            .next()
            .unwrap()
            .to_string();
        let version = match x
            .next()
            .unwrap()
        {
            "HTTP/1.1" => HttpVersion::V1_1,
            _ => HttpVersion::V1_1,
        };
        RequestLine {
            method,
            target,
            version,
        }
    }
}

impl From<&[u8]> for Request {
    fn from(bytes: &[u8]) -> Request {
        let request = String::from_utf8(bytes.to_vec())
            .expect(
                "Could not convert requset to UTF8 string.",
            );
        let (request_line, request_remaining) = request
            .split_once("\r\n")
            .unwrap();
        let request_line: RequestLine = request_line.into();
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
        println!(
            "{:#?}",
            headers
        );
        let body = body.to_string();

        Request {
            request,
            request_line,
            headers,
            body,
        }
    }
}
