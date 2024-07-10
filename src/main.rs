use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() -> std::io::Result<()> {
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
                let mut request_buffer = [0_u8; 1024];
                let request_buffer_len = open_stream
                    .read(&mut request_buffer)?;
                let response = if request_buffer
                    .starts_with(b"GET / HTTP")
                {
                    "HTTP/1.1 200 OK\r\n\r\n".to_string()
                } else if request_buffer
                    .starts_with(b"GET /user-agent")
                {
                    let body = match std::str::from_utf8(
                        &request_buffer,
                    ) {
                        Ok(request) => request
                            .split("\r\n")
                            .find(|s| {
                                s.starts_with("User-Agent: ")
                            })
                            .unwrap_or("/echo/NothingFound")
                            .replace(
                                "User-Agent: ", "",
                            ),
                        Err(e) => format!(
                            "Error {}",
                            e
                        ),
                    };
                    let body_len = body.len();
                    format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", body_len, body)
                } else if request_buffer
                    .starts_with(b"GET /echo/")
                {
                    let body = match std::str::from_utf8(
                        &request_buffer,
                    ) {
                        Ok(request) => request
                            .split_ascii_whitespace()
                            .find(|s| {
                                s.starts_with("/echo/")
                            })
                            .unwrap_or("/echo/NothingFound")
                            .replace(
                                "/echo/", "",
                            ),
                        Err(e) => format!(
                            "Error {}",
                            e
                        ),
                    };
                    let body_len = body.len();
                    format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", body_len, body)
                } else {
                    "HTTP/1.1 404 Not Found\r\n\r\n"
                        .to_string()
                };
                open_stream
                    .write_all(response.as_bytes())?;
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
