use std::{
    io::{BufRead, BufReader, Result},
    net::{TcpListener, TcpStream},
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
    // let display_controller = DisplayController::new(25, 24, 23, 100);
    
    for line in reader.lines() {
        let data = line.expect("Failed to read line");
        let data = data.trim().to_string();
        println!("[receiver] New data: {}", data);

        match DisplayController::display(data) {
            Ok(()) => {},
            Err(e) => eprintln!("{}", e)
        };
    }

    println!("[receiver] Client disconnected");

    Ok(())
}
