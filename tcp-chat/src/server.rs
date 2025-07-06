use std::{
    net::TcpListener,
    thread,
};

mod handlers;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Could not bind");

    println!("🚀 Server listening on 127.0.0.1:8000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("✅ Connection Established: {:?}", stream.peer_addr().unwrap());

                thread::spawn(move || {
                    handlers::handle_client(stream);
                });
                // handlers::handle_connection(stream);
            }
            Err(e) => {
                eprintln!("❌ Connection failed: {}", e);
            }
        }
    }
}
