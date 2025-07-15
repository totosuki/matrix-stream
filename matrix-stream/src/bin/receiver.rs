use std::{
    io::{BufRead, BufReader, Result},
    net::{TcpListener, TcpStream},
};

use matrix_stream::display_controller::{self, DisplayController};

fn main() -> Result<()> {
    println!("[receiver] start");

    start_server("0.0.0.0", 8080)?;

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
               if let Err(e) = handle_client(stream) {
                    eprintln!("[receiver] Client handling error: {}", e);
                }
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
    let mut display_controller = DisplayController::new();
    display_controller.start(25, 24, 23, 10)?;

    for line in reader.lines() {
        let line = line?;
        if let Err(e) = display_controller.update_data(line) {
            eprintln!("[receiver] Failed to update display data: {}", e);
            continue;
        }
    }

    println!("[receiver] Client disconnected");

    display_controller.stop()?;

    Ok(())
}
