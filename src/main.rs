use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use budget_forecast::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(12);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (method, path) = extract_route(&request_line[..]);

    let handler = match (method, path) {
        ("GET", "/") => Routes::index,
        ("GET", "/styles.css") => Routes::styles,
        ("GET", "/scripts.js") => Routes::script,
        ("GET", "/favicon.ico") => Routes::favicon,
        _ => Routes::not_found,
    };

    handler(stream);
}

const OK_200: &str = "HTTP/1.1 200 OK";
const NOT_FOUND_400: &str = "HTTP/1.1 404 NOT FOUND";

struct Routes {}
impl Routes {
    fn header(status: &str, content_type: &str, content_length: usize) -> String {
        format!(
            "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
            status, content_type, content_length
        )
    }

    fn index(mut stream: TcpStream) {
        let contents = fs::read("client/index.html").unwrap();
        let response = Self::header(OK_200, "text/html", contents.len());

        stream.write_all(response.as_bytes()).unwrap();
        stream.write_all(&contents).unwrap();
    }

    fn script(mut stream: TcpStream) {
        let contents = fs::read("client/scripts.js").unwrap();
        let response = Self::header(OK_200, "application/javascript", contents.len());

        stream.write_all(response.as_bytes()).unwrap();
        stream.write_all(&contents).unwrap();
    }

    fn styles(mut stream: TcpStream) {
        let contents = fs::read("client/styles.css").unwrap();
        let response = Self::header(OK_200, "text/css", contents.len());

        stream.write_all(response.as_bytes()).unwrap();
        stream.write_all(&contents).unwrap();
    }

    fn favicon(mut stream: TcpStream) {
        let contents = fs::read("client/favicon.ico").unwrap();
        let response = Self::header(OK_200, "image/x-icon", contents.len());

        stream.write_all(response.as_bytes()).unwrap();
        stream.write_all(&contents).unwrap();
    }

    fn not_found(mut stream: TcpStream) {
        let contents = fs::read("client/404.html").unwrap();
        let response = Self::header(NOT_FOUND_400, "text/html", contents.len());

        stream.write_all(response.as_bytes()).unwrap();
        stream.write_all(&contents).unwrap();
    }
}

fn extract_route(request: &str) -> (&str, &str) {
    let params: Vec<&str> = request.split(' ').collect();

    (params[0], params[1])
}
