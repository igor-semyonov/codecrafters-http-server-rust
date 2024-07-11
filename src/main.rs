use std::{
    io::{Read, Write},
    net::TcpListener,
};

mod http;
use http::*;

const REQUEST_BUFFEX_SIZE: usize = 512;

fn main() -> std::io::Result<()> {
    println!("Logs from your program will appear here!");

    let listener =
        TcpListener::bind("127.0.0.1:4221").unwrap();

    'stream: for stream in listener.incoming() {
        match stream {
            Ok(mut open_stream) => {
                println!(
                    "accepted new connection from {}",
                    open_stream.peer_addr()?
                );
                let mut request_buffer =
                    [0_u8; REQUEST_BUFFEX_SIZE];
                let request_buffer_len = open_stream
                    .read(&mut request_buffer)?;
                if request_buffer_len > REQUEST_BUFFEX_SIZE
                {
                    // received request that exceeds buffer
                    // length
                    continue 'stream;
                }
                let request_buffer =
                    &request_buffer[0..request_buffer_len];
                let request: Request =
                    request_buffer.into();

                let response = if request.method
                    == HttpMethod::Get
                    && request.target == "/"
                {
                    Response {
                        version: HttpVersion::V1_1,
                        code: ResponseCode::C200,
                        headers:
                            std::collections::HashMap::new(),
                        body: "".to_string(),
                    }
                } else if request.method == HttpMethod::Get
                    && request
                        .target
                        .starts_with("/user-agent")
                {
                    let body = request
                        .headers
                        .get("User-Agent")
                        .unwrap()
                        .to_string();
                    let mut headers =
                        std::collections::HashMap::new();
                    headers.insert(
                        "Content-Length".to_string(),
                        body.as_bytes()
                            .len()
                            .to_string(),
                    );
                    headers.insert(
                        "Content-Type".to_string(),
                        "text/plain".to_string(),
                    );
                    Response {
                        version: HttpVersion::V1_1,
                        code: ResponseCode::C200,
                        headers,
                        body,
                    }
                } else if request.method == HttpMethod::Get
                    && request
                        .target
                        .starts_with("/echo/")
                {
                    let body = request
                        .target
                        .trim_start_matches("/echo/")
                        .to_string();
                    let mut headers =
                        std::collections::HashMap::new();
                    headers.insert(
                        "Content-Length".to_string(),
                        body.as_bytes()
                            .len()
                            .to_string(),
                    );
                    headers.insert(
                        "Content-Type".to_string(),
                        "text/plain".to_string(),
                    );
                    Response {
                        version: HttpVersion::V1_1,
                        code: ResponseCode::C200,
                        headers,
                        body,
                    }
                } else {
                    let headers = std::collections::HashMap::new();
                    Response {
                        version: HttpVersion::V1_1,
                        code: ResponseCode::C404,
                        headers,
                        body: "".to_string(),
                    }
                };
                let s: String = response.into();
                println!("{}", s);
                open_stream
                    .write_all(s.as_bytes())?;
            }
            Err(e) => {
                println!(
                    "error: {}",
                    e
                );
            }
        }
    }

    Ok(())
}
