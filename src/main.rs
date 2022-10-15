fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    address: String,
}

impl Server {
    fn new(addr: String) -> Server {
        Server {
            address: addr,
        }
    }

    fn run(self) {
        println!("Listening on {}", self.address);
    }
}

