use std::borrow::BorrowMut;
use std::net::{IpAddr, TcpListener};
use std::io::BufRead;
use local_ip_address::local_ip;
// GET / HTTP/1.1
// Host: 10.0.0.207:9999
// Connection: keep-alive
// Upgrade-Insecure-Requests: 1
// User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7
// Accept-Encoding: gzip, deflate
// Accept-Language: en-US,en;q=0.9
struct Header
{
    key :String,
    value:String

}

fn main() 
{
    let my_local_ip: IpAddr = local_ip().unwrap();
    println!("Hello, server!");
    println!("http://{my_local_ip}:9999 is my ip!");
    
    let listener:TcpListener =  std::net::TcpListener::bind(format!("{my_local_ip}:9999")).unwrap();


    for stream in listener.incoming()
    {
        let mut reader = std::io::BufReader::new(stream.unwrap());


        let mut line = String::new();
        loop {
            let resonse_size:  usize  = reader.read_line( &mut line).unwrap();
            if  resonse_size > 0
            {

                //GET / HTTP/1.0\r\n
                if line.contains("GET")
                {
                    // / HTTP/1.0\r\n
                }
                else 
                {   //must be a header than ...
                    let mut header_parts:Vec<&str> = line.split(":").collect();

                    let mut val:String = String::new();
                    for mut line_to_append in header_parts
                    {

                    }
                    let mut header_parts:Vec<&str> = line.split(":").collect();

                    let  _header :Header = Header
                    {
                        key     :   header_parts[0].to_string(), 
                        value   :   String::from("")
                    };

                }
                print!("{line}");

            }

        }
        
    }
}
