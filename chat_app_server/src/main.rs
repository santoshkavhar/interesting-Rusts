use std::thread;
use std::io::{Write, Error, BufReader, BufRead};
use std::net::{TcpStream, TcpListener};

const LOCAL:&str = "0.0.0.0:3333";

fn handle_client(mut stream : TcpStream) -> Result<(), Error> {
    println!("Incoming connection from : {}", stream.peer_addr()? );

    loop{
        // Taking these out sets out some default value every time!
        let mut input = String::new();
        let mut buf: Vec<u8> = Vec::new();
        let mut reader = BufReader::new(&stream);

        reader.read_until(b'\n', &mut buf).expect("Failed to read into buffer");

        println!("From client: {}", std::str::from_utf8(&buf).expect("Failed to convert to utf-8") );
        
        std::io::stdin().read_line(&mut input).expect("Couldn't read!");

        stream.write(input.as_bytes()).expect("Failed to write!");
    }
}

fn main(){
    let listener = TcpListener::bind(LOCAL).expect("Couldn't bind to port");
    for stream in listener.incoming(){

        match stream{
            Err(e) => { eprintln!("failed: {}", e)}
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream)    
                });
            }
        }
    }
}
