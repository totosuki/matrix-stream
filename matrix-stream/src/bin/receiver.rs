use std::{
    io::{BufRead, BufReader, Result},
    net::{TcpListener, TcpStream},
    thread,
    sync::{Arc, Mutex},
};

use matrix_stream::display_controller::DisplayController;

fn main() -> Result<()> {
    println!("[receiver] start");

    start_server("localhost", 8080)?;

    println!("[receiver] stop");
    Ok(())
}

fn start_server(bind_addr: &str, port: u16) -> Result<()> {
    // Bind
    let listener = TcpListener::bind(format!("{}:{}", bind_addr, port))?;
    println!("[receiver] Listening on {}:{}", bind_addr, port);

    // Listen
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("[receiver] New connection: {}", stream.peer_addr()?);
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("[receiver] Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(stream: TcpStream) -> Result<()> {
    let reader = BufReader::new(stream);
    let shared_data = Arc::new(Mutex::new("0".repeat(64).to_string()));

    DisplayController::display(25, 24, 23, 10, shared_data.clone());

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut data = shared_data.lock().unwrap();
        *data = line.trim().to_string();
        println!("[receiver] New data: {}", *data);
    }

    println!("[receiver] Client disconnected");

    Ok(())
}
