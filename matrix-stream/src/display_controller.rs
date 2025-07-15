use std::{
    io::{Error, ErrorKind, Result},
    thread::{self, JoinHandle},
    sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}},
    time::Duration,
};

#[cfg(feature = "raspi")]
use crate::drivers::osl641505::Osl641505;
use crate::protocol::ProtocolParser;

pub struct DisplayController {
    shared_data:  Arc<Mutex<String>>,
    running: Arc<AtomicBool>,
    thread_handle: Option<JoinHandle<()>>,
}

impl DisplayController {
    pub fn new() -> Self {
        Self {
            shared_data: Arc::new(Mutex::new("0".repeat(64))),
            running: Arc::new(AtomicBool::new(false)),
            thread_handle: None,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    pub fn start(
        &mut self,
        data_pin: u8,
        latch_pin: u8,
        clock_pin: u8,
        duration: u64,
    ) -> Result<()> {
        if self.is_running() {
            eprintln!("[DisplayController] Already running");
            return Ok(());
        }

        println!("[DisplayController] Starting display");

        self.running.store(true, Ordering::Relaxed);
        let shared_data = Arc::clone(&self.shared_data);
        let running = Arc::clone(&self.running);
        let mut led_matrix = Osl641505::new(data_pin, latch_pin, clock_pin, duration);

        let handle = thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                let data = {
                    let guard = match shared_data.lock() {
                        Ok(guard) => guard,
                        Err(e) => {
                            eprintln!("[DisplayController] Failed to acquire lock : {}", e);
                            thread::sleep(Duration::from_millis(10));
                            continue;
                        }
                    };
                    guard.clone()
                };

                if let Err(e) = ProtocolParser::validate_frame_data(&data) {
                    eprintln!("[DisplayController] Validation error : {}", e);
                    thread::sleep(Duration::from_millis(10));
                    continue;
                };

                match ProtocolParser::parse_frame_data(data) {
                    Some(data) => {
                        if let Err(e) = led_matrix.draw(data) {
                            eprintln!("[DisplayController] Draw error : {}", e);
                        }
                    },
                    None => {
                        eprintln!("[DisplayController] can not parse data");
                        continue;
                    },
                };

                // CPU負荷軽減
                thread::sleep(Duration::from_millis(1));
            }

            if let Err(e) = led_matrix.reset() {
                eprintln!("[DisplayController] Failed to reset LED matrix : {}", e);
            }

            println!("[DisplayController] Display thread terminated");
        });

        self.thread_handle = Some(handle);
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        if !self.is_running() {
            return Ok(())
        }

        println!("[DisplayController] Stopping display");

        self.running.store(false, Ordering::Relaxed);

        if let Some(handle) = self.thread_handle.take() {
            match handle.join() {
                Ok(_) => println!("[DisplayController] Display thread joined successfully"),
                Err(e) => eprintln!("[DisplayController] Failed to join display thread: {:?}", e),
            }
        }

        Ok(())
    }

    pub fn update_data(&self, new_data: String) -> Result<()> {
        if !self.is_running() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "DisplayController is not running"
            ));
        }

        match self.shared_data.lock() {
            Ok(mut data) => {
                *data = new_data.trim().to_string();
                println!("[DisplayController] Data updated : {}", *data);
                Ok(())
            },
            Err(e) => {
                Err(Error::new(
                    ErrorKind::Other,
                    format!("Failed to update data : {}", e)
                ))
            }
        }
    }
}

impl Drop for DisplayController {
    fn drop(&mut self) {
        if self.is_running() {
            let _ = self.stop();
        }
    }
}