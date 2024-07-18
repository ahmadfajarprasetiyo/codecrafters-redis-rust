mod string_redis;

use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::sync::Mutex;
use std::thread;
use lazy_static::lazy_static;
use string_redis::simple_string;
use crate::string_redis::bulk_string;

lazy_static! {
    static ref INSTANCE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

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

        let payloads = parsing_payload(&req);
        if payloads.len() < 1 {
            return;
        }

        println!("{:?}", payloads);

        let mut return_string = String::new();
        if payloads.first().unwrap().eq("PING") || payloads.first().unwrap().eq("COMMAND") {
            return_string = simple_string("PONG");
        } else if payloads.first().unwrap().eq("ECHO") {
            return_string = simple_string(payloads.last().unwrap());
        } else if payloads.first().unwrap().eq("SET") {
            {
                let mut instance = INSTANCE.lock().unwrap();
                instance.insert(payloads.get(1).unwrap().to_string(), payloads.get(2).unwrap().to_string());
            }

            return_string = simple_string("OK");
        } else if payloads.first().unwrap().eq("GET") {
            return_string = {
                let lock = INSTANCE.lock().unwrap();
                let key = payloads.get(1).unwrap().to_string();
                let val = lock.get(&key).unwrap();

                val.to_owned()
            };

            return_string = bulk_string(&return_string);
        }


        stream.write(return_string.as_bytes()).unwrap();
        
    }
}

fn parsing_payload(payload: &String) -> Vec<String> {
    let cmds = payload.split_whitespace();
    let mut res = vec![];

    for cmd in cmds {
        if !cmd.starts_with('*') && !cmd.starts_with('$') {
            res.push(cmd.to_string());
        }

    }

    res
}