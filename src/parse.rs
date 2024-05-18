static  SEPERATORS: &'static str = "()[]{}/=+-,.:";
use std::fs::{self};
use std::io::{prelude::*, ErrorKind};
use std::net::TcpStream;
use  std::str::*;
pub fn parse_into_tokens(contents : String) -> Vec<String>{
    let mut tokens: Vec<String> = vec![];
    let mut str_buffer: String = String::from("");
    for character in contents.chars(){
        for sep in SEPERATORS.chars(){
            if sep == character{
                tokens.push(str_buffer.clone());
                tokens.push(sep.to_string());
                str_buffer.clear();
            }
        }
        if character.is_alphanumeric() {
            str_buffer += &character.to_string();

        }
    }
    return tokens;
}

pub fn read_and_handle_errors(stream: &mut TcpStream) {
    let mut whole_req_bytes:[u8;1024] = [0; 1024];

    match stream.read(&mut whole_req_bytes) {

        Ok(_bytes_read) => {
            on_ok_client_connect(stream, &mut whole_req_bytes);
            print!("Ok, handling request!");
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
pub fn on_ok_client_connect(stream: & mut TcpStream, whole_req_bytes:&mut [u8;1024])
{
    let whole_req_str: &str =     from_utf8(whole_req_bytes).unwrap();

    let line_vec: Vec<_> = whole_req_str.split('\n').collect();
    for line in line_vec
    {
        //each line can be header or get request. 
        let header_tokized = parse_into_tokens(line.to_string());
        if  header_tokized.len() > 0
        {
            if header_tokized[0] == "GET"
            {

                respond_to_get(stream);
            }
        }
    }
}
pub fn respond_to_get(stream: &mut TcpStream )
{

    let contents = fs::read_to_string("./hello.html").unwrap();
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
