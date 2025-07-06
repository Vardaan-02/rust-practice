use std::{io::{BufRead, BufReader, Read, Write}, net::TcpStream};

// tcp request
pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected.");
                break;
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        };

        let received = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received: {}", received); 

        let response = format!("Server got: {}", received);
        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to write to client: {}", e);
        }
    }
}


// http request
pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{:?}",http_request);

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}