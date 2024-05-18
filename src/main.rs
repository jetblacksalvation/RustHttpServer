use local_ip_address::local_ip;
use std::fs::{self};
use std::io::{prelude::*, ErrorKind};
use std::net::{IpAddr, TcpListener,TcpStream};
use  std::str::*;
// GET / HTTP/1.1
// Host: 10.0.0.207:9999
// Connection: keep-alive
// Upgrade-Insecure-Requests: 1
// User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7
// Accept-Encoding: gzip, deflate
// Accept-Language: en-US,en;q=0.9
struct Header {
    key: String,
    value: String,
}
fn read_and_handle_errors(stream: &mut TcpStream) {
    let mut whole_req_bytes = [0; 1024];

    match stream.read(&mut whole_req_bytes) {

        Ok(_bytes_read) => {
            print!("Ok, handling request!");
            let whole_req_str =     from_utf8(&mut whole_req_bytes).unwrap();  
            println!("{whole_req_str}");
            let line_vec: Vec<_> = whole_req_str.split('\n').collect();
            for line in line_vec {
                //GET / HTTP/1.0\r\n
                // println!("is line - {line}");
                read_the_client_stream(stream, line.clone());
                if line.is_empty()
                {
                    println!("Line is empty!");
                }
                else {
                    // print!("in header parser line = {line}");
                    //must be a header than ...
                    let header_parts: Vec<&str> = line.split(':').collect();
                    let mut key_slice: Vec<&str> = Vec::new();
                    let mut val: String = String::new();
                    if header_parts.len() > 2 {
                        key_slice.extend_from_slice(&header_parts[1..]);

                        for line_to_append in key_slice {
                            val += line_to_append;
                        }
                        let header_parts: Vec<&str> = line.split(":").collect();

                        let header: Header = Header {
                            key: header_parts[0].to_string(),
                            value: val,
                        };
                        let key = header.key;
                        let val = header.value;
                        println!("Header : key - {key}, val - {val}")
                    }
                }
            
            }
        }
        Err(e) if e.kind() == ErrorKind::ConnectionAborted => {
            println!("Other side disconnected");
        }
        Err(e) => {
            println!("Some other error occurred: {e}");
        }
    }
    println!("done reading");
    return;

}
fn read_the_client_stream(stream: &mut TcpStream, line: &str, ){
    let contents = fs::read_to_string("./hello.html").unwrap();

    if line.contains("GET") {
        // / HTTP/1.0\r\n
        // let mut stream = TcpStream::connect(format!("{my_local_ip}:9999")).unwrap();
        
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        println!("Line has get!");
    } 
    else{
        println!("{line} was not get!!")

    }
}
fn main() {
    let my_local_ip: IpAddr = local_ip().unwrap();
    println!("Hello, server!");
    println!("http://{my_local_ip}:9999 is my ip!");

    let listener: TcpListener = std::net::TcpListener::bind(format!("{my_local_ip}:9999")).unwrap();
    for stream in listener.incoming() {
        println!("readandhandle");
        read_and_handle_errors(&mut stream.unwrap())
    }
}
