use std::{
    io::Result,
    time::Duration,
    thread,
};

use super::hc595::Hc595;
use crate::Level;

pub struct Osl641505 {
    shift_register: Hc595,
    duration: Duration
}

impl Osl641505 {
    pub fn new(data_pin: u8, latch_pin: u8, clock_pin: u8, duration: u64) -> Self {
        Self {
            shift_register: Hc595::new(data_pin, latch_pin, clock_pin),
            duration: Duration::from_millis(duration),
        }
    }

    pub fn draw(&mut self, data: &u64) -> Result<()> {
        for i in 0..8 {
            for j in 0..8 {
                let bit = (data >> (i * 8 + j)) & 1;
                if bit == 1 {
                    self.set_pixel((i-7)*-1, (j-7)*-1)?;
                }
            }
        }
        Ok(())
    }

    pub fn reset(&mut self) -> Result<()> {
        self.shift_register.reset(Level::Low)?;
        Ok(())
    }

    fn set_pixel(&mut self, row: i32, col:i32) -> Result<()> {
        let data= ((1 << col + 8) + (1 << row)) ^ 0xFF00;
        self.shift_register.write(data)?;
        self.shift_register.latch()?;
        thread::sleep(self.duration);
        Ok(())
    }
}