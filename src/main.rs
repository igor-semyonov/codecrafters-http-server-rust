use clap::Parser;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

mod http;
use http::*;

#[derive(Parser, Debug, Clone)]
struct Args {
    /// The root directory that files are served from
    #[arg(
        short, long
    )]
    directory: Option<std::path::PathBuf>,
}

const REQUEST_BUFFEX_SIZE: usize = 512;

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    println!("Logs from your program will appear here!");

    let listener =
        TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut open_stream) => {
                println!(
                    "accepted new connection from {}",
                    open_stream.peer_addr()?
                );
                // handle_connection(&mut open_stream);
                let args = args.clone();
                std::thread::spawn(
                    move || {
                        handle_connection(
                            &mut open_stream,
                            args,
                        )
                    },
                );
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

fn handle_connection(
    open_stream: &mut TcpStream,
    args: Args,
) -> std::io::Result<()> {
    let mut request_buffer = [0_u8; REQUEST_BUFFEX_SIZE];
    let request_buffer_len =
        open_stream.read(&mut request_buffer)?;
    if request_buffer_len > REQUEST_BUFFEX_SIZE {
        // received request that exceeds buffer
        // length
        return Ok(());
    }
    let request_buffer =
        &request_buffer[0..request_buffer_len];
    let request: Request = request_buffer.into();

    let response = if request.method == HttpMethod::Get
        && request.target == "/"
    {
        Response {
            version: HttpVersion::V1_1,
            code: ResponseCode::C200,
            headers: std::collections::HashMap::new(),
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
        let mut headers = std::collections::HashMap::new();
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
        let mut headers = std::collections::HashMap::new();
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
    } else if args
        .directory
        .is_some()
        && request.method == HttpMethod::Get
        && request
            .target
            .starts_with("/files/")
    {
        let mut file = args
            .directory
            .unwrap()
            .clone();
        file.push(
            request
                .target
                .trim_start_matches("/files/"),
        );
        let body = std::fs::read_to_string(file);

        match body {
            Ok(body) => {
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
                    "application/octet-stream".to_string(),
                );
                Response {
                    version: HttpVersion::V1_1,
                    code: ResponseCode::C200,
                    headers,
                    body,
                }
            }
            Err(_) => {
                let headers =
                    std::collections::HashMap::new();
                Response {
                    version: HttpVersion::V1_1,
                    code: ResponseCode::C404,
                    headers,
                    body: "".to_string(),
                }
            }
        }
    } else if request.method == HttpMethod::Post
        && request
            .target
            .starts_with("/files/")
    {
        let mut file = args
            .directory
            .unwrap()
            .clone();
        file.push(
            request
                .target
                .trim_start_matches("/files/"),
        );
        let headers = std::collections::HashMap::new();
        match std::fs::write(
            file,
            &request.body,
        ) {
            Ok(_) => Response {
                version: HttpVersion::V1_1,
                code: ResponseCode::C201,
                headers,
                body: "".to_string(),
            },
            Err(e) => Response {
                version: HttpVersion::V1_1,
                code: ResponseCode::C409,
                headers,
                body: format!(
                    "Encountered error {}",
                    e
                ),
            },
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
    let s: String = response
        .clone()
        .into();
    let s_bytes = s.as_bytes();
    // println!(
    //     "{:#?}",
    //     &request
    // );
    // println!(
    //     "{:#?}",
    //     &response
    // );
    open_stream.write_all(s_bytes)?;

    Ok(())
}
