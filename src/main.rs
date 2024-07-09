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
                open_stream
                    .read(&mut request_buffer)?;
                let response = if request_buffer
                    .starts_with(b"GET / HTTP")
                {
                    "HTTP/1.1 200 OK\r\n\r\n"
                } else {
                    "HTTP/1.1 404 Not Found\r\n\r\n"
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
