// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read,};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_client(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf).expect("Failed to read from client");
        let req = match String::from_utf8(buf[..bytes_read].into()) {
            Ok(req) => req,
            Err(e) => {
                eprintln!("failed to parse request as utf8: {e}");
                return;
            }
        };

        println!("{}", req);
        if req.contains("PING") {
            stream.write("+PONG\r\n".as_bytes()).unwrap();
        }

        if bytes_read == 0 {
            return;
        }

    }
}