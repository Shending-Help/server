use std::io::Read;
use std::net::TcpListener;

use crate::http::request;
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
                            match request::Request::try_from(&buf[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                }
                                Err(e) => {
                                    println!("failed to parse: {}", e);
                                }
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
