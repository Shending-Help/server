use std::io::Read;
use std::net::TcpListener;

use crate::http::{ParseError, Request, Response, StatusCode};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(addr: String) -> Server {
        Server { address: addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Request is: {}", String::from_utf8_lossy(&buf));
                            let response = match Request::try_from(&buf[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => {
                                    println!("failed to parse: {}", e);
                                    handler.handle_bad_request(&e)
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("failed to send response: {}", e);
                            }
                        }

                        Err(e) => {
                            println!("error: {}", e)
                        }
                    }
                }

                Err(e) => {
                    println!("failed to establish a connection: {}", e);
                }
            }
        }
    }
}
