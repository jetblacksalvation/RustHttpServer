

use local_ip_address::local_ip;

mod parse;

use crate::parse::*;
use std::net::{IpAddr, TcpListener};

// GET / HTTP/1.1
// Host: 10.0.0.207:9999
// Connection: keep-alive
// Upgrade-Insecure-Requests: 1
// User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7
// Accept-Encoding: gzip, deflate
// Accept-Language: en-US,en;q=0.9
// struct Header {
//     key: String,
//     value: String,
// }
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
