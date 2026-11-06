//! A local HTTP/1.1 server for download tests; every connection gets one answer and is closed.

use std::io::{BufRead as _, BufReader, Write as _};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub enum Reply {
    Full(u16, Vec<u8>),
    /// Announces `length` bytes, sends `sent`, then nothing more.
    Stall {
        length: usize,
        sent: Vec<u8>,
    },
    /// Sends `body` in pieces of `chunk` bytes with a pause after each.
    Trickle {
        body: Vec<u8>,
        chunk: usize,
        pause: Duration,
    },
}

pub struct Server {
    pub base_url: String,
    requests: Arc<Mutex<Vec<String>>>,
}

impl Server {
    /// The request targets in the order they arrived.
    pub fn requests(&self) -> Vec<String> {
        self.requests.lock().unwrap().clone()
    }
}

pub fn serve(route: impl Fn(&str) -> Reply + Send + Sync + 'static) -> Server {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base_url = format!("http://{}", listener.local_addr().unwrap());
    let requests = Arc::new(Mutex::new(Vec::new()));
    let route = Arc::new(route);
    {
        let requests = Arc::clone(&requests);
        std::thread::spawn(move || {
            for stream in listener.incoming().flatten() {
                let route = Arc::clone(&route);
                let requests = Arc::clone(&requests);
                std::thread::spawn(move || answer(stream, &*route, &requests));
            }
        });
    }
    Server { base_url, requests }
}

fn answer(mut stream: TcpStream, route: &dyn Fn(&str) -> Reply, requests: &Mutex<Vec<String>>) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() {
        return;
    }
    loop {
        let mut header = String::new();
        match reader.read_line(&mut header) {
            Ok(0) | Err(_) => return,
            Ok(_) if header == "\r\n" => break,
            Ok(_) => {}
        }
    }
    let target = request_line
        .split_whitespace()
        .nth(1)
        .unwrap_or_default()
        .to_owned();
    requests.lock().unwrap().push(target.clone());
    let head = |status: u16, length: usize| {
        format!("HTTP/1.1 {status} X\r\nContent-Length: {length}\r\nConnection: close\r\n\r\n")
    };
    let _ = match route(&target) {
        Reply::Full(status, body) => stream
            .write_all(head(status, body.len()).as_bytes())
            .and_then(|()| stream.write_all(&body)),
        Reply::Stall { length, sent } => {
            let result = stream
                .write_all(head(200, length).as_bytes())
                .and_then(|()| stream.write_all(&sent))
                .and_then(|()| stream.flush());
            std::thread::sleep(Duration::from_secs(10));
            result
        }
        Reply::Trickle { body, chunk, pause } => stream
            .write_all(head(200, body.len()).as_bytes())
            .and_then(|()| {
                body.chunks(chunk).try_for_each(|piece| {
                    stream.write_all(piece)?;
                    stream.flush()?;
                    std::thread::sleep(pause);
                    Ok(())
                })
            }),
    };
}
