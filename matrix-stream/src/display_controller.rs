use std::{
    io::Result,
    thread,
    sync::{Arc, Mutex},
};

use crate::drivers::osl641505::Osl641505;
use crate::protocol::ProtocolParser;

pub struct DisplayController;

impl DisplayController {
    pub fn display(
        data_pin: u8,
        latch_pin: u8,
        clock_pin: u8,
        duration: u64,
        shared_data: Arc<Mutex<String>>
    ) -> Result<()> {
        println!("[DisplayController] display");

        let mut led_matrix = Osl641505::new(data_pin, latch_pin, clock_pin, duration);

        let _ = thread::spawn(move || {
            loop {
                let data = {
                    let tmp = shared_data.lock().unwrap();
                    tmp.clone()
                };

                match ProtocolParser::validate_frame_data(&data) {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("[DisplayController] Validation error: {}", e);
                        continue;
                    }
                };

                match ProtocolParser::parse_frame_data(data) {
                    Some(data) => {
                        led_matrix.draw(data);
                    },
                    None => {
                        eprintln!("[DisplayController] can not parse data");
                        continue;
                    },
                };
            }
        });

        Ok(())
    }
}