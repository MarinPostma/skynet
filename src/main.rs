use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use std::thread;

type Sid = [u8; 16];

lazy_static! {
    static ref HOSTS: Mutex<HashMap<Sid, Host>> = Mutex::new(HashMap::new());
}

#[derive(Serialize, Deserialize, Debug)]
struct HostInit {
    sid: Sid,
    pub_addr: SocketAddr,
}

struct Host {
    channel: Sender<Self>,
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

fn handle_client(mut stream: TcpStream) {
    thread::spawn(move || {
        let (tx, rx) = channel();
        let pub_addr = stream.peer_addr().expect("error retrieving peer id");
        let host = HostInit::new(pub_addr);
        let payload = serde_json::to_string(&host).expect("Serialize error");
        {
            HOSTS.lock().unwrap().insert(
                host.sid,
                Host {
                    channel: tx,
                    pub_addr,
                },
            );
        }
        stream.write(&payload.as_bytes());
        loop {}
    });
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7800").expect("error opening socket");
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
