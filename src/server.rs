use crate::http::Request;
use log::{info, warn};
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        info!("Initializing server. Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => match Request::try_from(&buffer[..]) {
                            Ok(request) => {
                                info!("Received request to the following path: {}", request.path)
                            }
                            Err(e) => {
                                warn!("Failed to parse a request: {}", e);
                            }
                        },
                        Err(e) => {
                            warn!("Failed to read from connection: {}", e)
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to establish connection: {}", e);
                }
            }
        }
    }
}
