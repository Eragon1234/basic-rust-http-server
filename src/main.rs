fn main() {
    let server = Server::new("localhost:8080".to_string());

    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self {
            addr,
        }
    }

    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}

struct Request {
    method: Method,
    path: String,
    query_string: Option<String>
}

enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    TRACE,
    PATCH,
    CONNECT,
}