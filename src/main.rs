use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::net::{TcpListener, TcpStream};
use std::thread;

#[derive(Serialize, Deserialize, Debug)]
struct HostInit {
    sid: [u8; 16],
    pub_addr: SocketAddr,
}

impl HostInit {
    fn new(pub_addr: SocketAddr) -> Self {
        let mut res = Self {
            sid: [0; 16],
            pub_addr,
        };
        thread_rng().fill(&mut res.sid);
        res
    }
}

fn handle_client(stream: TcpStream) {
    thread::spawn(move || {
        let public_addr = stream.peer_addr().unwrap();
        let host = HostInit::new(public_addr);
        let payload = serde_json::to_string(&host).unwrap();
        println!("payload: {}", payload);
        loop {
            stream.write(&payload.as_bytes());
        }
    });
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:57901").unwrap();
    listener.set_nonblocking(true).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                handle_client(s);
            }
            _ => {}
        }
    }
}
