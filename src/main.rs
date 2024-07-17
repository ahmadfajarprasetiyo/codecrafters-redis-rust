// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::thread;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


fn handle_client(mut stream: TcpStream) {
    // stream.write("+PONG\r\n".as_bytes()).unwrap();
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf).expect("Failed to read from client");
        if bytes_read == 0 {
            return;
        }

        let req = match String::from_utf8(buf[..bytes_read].into()) {
            Ok(req) => req,
            Err(e) => {
                println!("failed to parse request as utf8: {e}");
                return;
            }
        };

        println!("{}", req);
        if req.contains("PING") {
            stream.write("+PONG\r\n".as_bytes()).unwrap();
        } else if req.contains("ECHO") {

            let res = req.split_whitespace().last().unwrap();
            let res = "+".to_owned() + res;
            let res = res + "\r\n";
            stream.write(res.as_bytes()).unwrap();
        }
        
    }
}