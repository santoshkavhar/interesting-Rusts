use std::net::TcpStream;
use std::io::{self, BufReader, BufRead, Write};

const LOCAL:&str = "0.0.0.0:3333";

fn main(){
    let mut stream = TcpStream::connect(LOCAL).expect("Couldn't connect!");
    loop{
        let mut input : String = String::new();
        let mut buf: Vec<u8> = Vec::new();
        io::stdin().read_line(&mut input).expect("Couldn't read!");

        stream.write(input.as_bytes()).expect("Failed to write!");

        let mut reader = BufReader::new(&stream);

        reader.read_until(b'\n', &mut buf).expect("Failed to read!");

        println!("From Server: {}", std::str::from_utf8(&buf).expect("Couldn't Write to buffer as string") );
    }    
}
