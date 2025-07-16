use std::io::{Write, BufReader, BufRead};
#[allow(unused_imports)]
use std::net::TcpListener;
use std::net::TcpStream;
use std::io;
use std::any;
use std::path::Path;
use std::fs;

fn handle_connection(mut stream: TcpStream) -> io::Result<()>  {
    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    let mut buf_reader = BufReader::new(&mut stream);
    let http_request : Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("http request : {:?}", http_request);
    let http_request_path : &Vec<&str> = &http_request[0].split_whitespace().collect();
    let mut path = http_request_path.get(1).copied().unwrap_or("/");
    // let user_agent : Vec<&str> = http_request[2].split_whitespace().collect();
    if http_request.len() > 2 {
        let user_agent_parts: Vec<&str> = http_request[2].split_whitespace().collect();
        println!("User-Agent parts: {:?}", user_agent_parts);
    } else {
        println!("No third header line in request");
    }

    // println!("http_request type is : {:?}", any::type_name_of_val(&http_request));
    // println!("Http request part 1 : {}", http_request_path[1]);
    // println!("User_agent : {:?}", user_agent);
    // let mut path : &str = http_request_path;
    if path == "/" {
        path = "index.html";
    }
    let file_path = Path::new(path);
    if file_path.exists(){
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string(file_path).unwrap();
        let length   = contents.len();
        let response = format!("{status_line}\r\nContent-length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes())?;
    } 
    else if path.starts_with("/echo/") {
        let echoed_str = &path["/echo/".len()..];
        let status_line = "HTTP/1.1 200 OK";
        let length = echoed_str.len();
        let response = format!("{status_line}\r\nContent-Type: text/plain\r\nContent-Length: {length}\r\n\r\n{echoed_str}");
        stream.write_all(response.as_bytes())?;
    }
    else {
        let status_line = "HTTP/1.1 404 Not Found\r\n\r\n";
        // let contents    = fs::read_to_string("404.html").unwrap();
        // let length = contents.len() ;
        // let response = format!("{status_line}\r\nContent-length: {length}\r\n\r\n{contents}");
        stream.write_all(status_line.as_bytes())?;
    }
    // let path_check = Path::new(http_request_path);
    //stream.write_all(response.as_bytes()).unwrap();
    //stream.write_all(response.as_bytes())?;
    Ok(())
}

/*
fn handle_http_request(mut request) {
    // let mut line : Vec<&str> = request.split("\r\n").collect();
}
*/
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    //
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Accepted new connections");
                handle_connection(stream);
                
            }       
            Err(e) => {
                println!("error: {}", e);
            }
        }

    }
}
