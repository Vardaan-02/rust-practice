use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

fn read_line() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    input.trim().to_string()
}

fn request(stream:&mut TcpStream) -> bool {
    let input = read_line();
        
    if input.trim().eq_ignore_ascii_case("exit") {
        println!("Exiting...");
        return false;
    }

    stream.write_all(input.as_bytes()).expect("Failed to write to server");

    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer).expect("Failed to read from server");

    println!("Server replied: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
    true
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8000").expect("Could not connect to server");

    println!("Connected to server!");
    

    loop {
        if !request(&mut stream) {
            break;
        }
    }
}
