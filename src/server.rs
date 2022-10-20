use std::io::Read;
use std::net::TcpListener;

use crate::http::{Request, Response, StatusCode};
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(addr: String) -> Server {
        Server { address: addr }
    }

    pub fn run(self) {
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
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(
                                        StatusCode::Ok,
                                        Some("<h1>I Made a server!!</h1>".to_string()),
                                    )
                                }
                                Err(e) => {
                                    println!("failed to parse: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
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
