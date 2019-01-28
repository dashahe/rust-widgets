use std::net::{TcpListener, TcpStream};
use std::io;
//use std::thread::spawn;
use std::net;

struct EchoServer;

impl EchoServer {

    pub fn run<F>(ip: &str, port: u16, request_handler: F) -> io::Result<()>
        where F: Fn(&mut TcpStream, &mut TcpStream) -> () {

        let listener = TcpListener::bind(format!("{}:{}", ip, port))?;
        loop {
            for stream in listener.incoming() {
                let mut read_stream = stream?;
                let mut write_stream = read_stream.try_clone()?;
                request_handler(&mut read_stream, &mut write_stream);
                read_stream.shutdown(net::Shutdown::Both)?;
                write_stream.shutdown(net::Shutdown::Both)?;
            }
        }
        Ok(())
    }
}

fn main() {
    fn request_handler(read: &mut TcpStream, write: &mut TcpStream) {
        io::copy(read, write).expect("error when copy stream");
        match read.peer_addr() {
            Ok(addr) => { println!("handle request from [{:?}]", addr); },
            Err(e) => { println!("error when get peer addr: {}", e); }
        }
    }
    EchoServer::run("localhost", 9999, request_handler)
        .expect("error when run echo server!");
}