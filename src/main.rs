use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

use budget_forecast::{Forecast, ThreadPool};

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
    let mut buf_reader = BufReader::new(&stream);

    let mut headers = Vec::new();
    let mut content_length = 0;

    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line).unwrap();

    // <!-- AI
    loop {
        let mut line = String::new();
        buf_reader.read_line(&mut line).unwrap();
        let trimmed = line.trim_end();

        if trimmed.is_empty() {
            break;
        }

        if let Some(cl) = trimmed.strip_prefix("Content-Length: ") {
            content_length = cl.trim().parse::<usize>().unwrap_or(0);
        }

        headers.push(trimmed.to_string());
    }

    let mut body = vec![0u8; content_length];
    buf_reader.read_exact(&mut body).unwrap();
    // AI -->

    let (method, path) = extract_route(&request_line[..]);

    match (method, path) {
        ("GET", "/") => Routes::index(stream),
        ("GET", "/styles.css") => Routes::styles(stream),
        ("GET", "/scripts.js") => Routes::script(stream),
        ("GET", "/favicon.ico") => Routes::favicon(stream),
        ("POST", "/api/forecast") => Routes::forecast(stream, body),
        _ => Routes::not_found(stream),
    };
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

    fn forecast(mut stream: TcpStream, body: Vec<u8>) {
        let forecast = Forecast::from(&body);
        let content = forecast.response();

        let response = Self::header(OK_200, "application/json", content.len());

        stream.write_all(response.as_bytes()).unwrap();
        stream.write_all(content.as_bytes()).unwrap();
    }
}

fn extract_route(request: &str) -> (&str, &str) {
    let params: Vec<&str> = request.split(' ').collect();

    (params[0], params[1])
}
