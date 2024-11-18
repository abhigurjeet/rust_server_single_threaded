use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000".to_string());
    match listener {
        Ok(tcp_listener) => {
            println!("{:?}", tcp_listener);
            for _i in 0..5 {
                let connection = tcp_listener.accept();
                match connection {
                    Ok(val) => {
                        let (mut tcp_stream, _client_addr): (TcpStream, SocketAddr) = val;
                        let data = BufReader::new(&mut tcp_stream);
                        let mut send_response: bool = false;
                        for line in data.lines() {
                            match line {
                                Ok(val) => {
                                    if "KIDDA / HTTP/1.1" == val {
                                        send_response = true;
                                    }
                                    break;
                                }
                                Err(error) => {
                                    println!("{}", error);
                                }
                            }
                        }
                        if send_response {
                            let file_read = fs::read_to_string("response/index.html").unwrap();
                            let length = file_read.len();
                            let response = format!(
                                "HTTP/1.1 200\r\nContent-Length: {length}\r\n\r\n{file_read}"
                            );
                            let res = tcp_stream.write_all(response.as_bytes());
                            match res {
                                Ok(_) => {
                                    println!("Response sent succesfully ");
                                }
                                Err(error) => {
                                    println!("Error sending response {}", error);
                                }
                            }
                        } else {
                            let file_read = fs::read_to_string("response/error.html").unwrap();
                            let length = file_read.len();
                            let response = format!(
                                "HTTP/1.1 404\r\nContent-Length: {length}\r\n\r\n{file_read}"
                            );
                            let res = tcp_stream.write_all(response.as_bytes());
                            match res {
                                Ok(_) => {
                                    println!("Response sent succesfully ");
                                }
                                Err(error) => {
                                    println!("Error sending response {}", error);
                                }
                            }
                        }
                    }
                    Err(error) => {
                        println!("Error: {}", error);
                    }
                }
            }
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}
