#![allow(unused)]
use std::io::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8090")?;

    stream.write(b"")?;
    loop {
        print!("data:{}\n", get_stream_str(&mut stream));
    }
    Ok(())
} 

fn get_stream_str(stream : &mut TcpStream) ->  String {
    let mut tcp_buffer: Vec<u8> = Vec::new();
    loop {
        let mut data: [u8;1] = [0;1];
        stream.read(&mut data).unwrap();
        if data[0] != 10 {
            tcp_buffer.push(data[0]);
        }
        else {
            break;
        }
    }
    return String::from_utf8_lossy(&tcp_buffer).to_string();
} 
