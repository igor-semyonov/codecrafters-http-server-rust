use std::{io::Write, net::TcpListener};

fn main() -> std::io::Result<()> {
    println!("Logs from your program will appear here!");

    let listener =
        TcpListener::bind("127.0.0.1:4221").unwrap();

    let response = b"HTTP/1.1 200 OK\\r\\n\\r\\n";
    for stream in listener.incoming() {
        match stream {
            Ok(mut open_stream) => {
                println!(
                    "accepted new connection from {}",
                    open_stream.peer_addr()?
                );
                open_stream
                    .write(response)?;
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
