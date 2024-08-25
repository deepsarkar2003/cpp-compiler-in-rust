
// src/server.rs
use tiny_http::{Server, Response};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn start_server(port: u16, root_dir: &str) {
    let server = Server::http(format!("0.0.0.0:{}", port)).unwrap();

    for request in server.incoming_requests() {
        let url = request.url();
        let path = if url == "/" {
            format!("{}/index.html", root_dir)
        } else {
            format!("{}/{}", root_dir, &url[1..])
        };

        let response = if Path::new(&path).exists() {
            let mut file = File::open(path).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            Response::from_string(content)
        } else {
            Response::from_string("404 Not Found").with_status_code(404)
        };

        request.respond(response).unwrap();
    }
}
