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
            duration: Duration::from_micros(duration),
        }
    }

    pub fn draw(&mut self, data: u64) -> Result<()> {
        for row in 0..8 {
            let row_data = self.extract_row_data(&data, row);

            self.display_row(row, row_data)?;

            thread::sleep(self.duration);
        }

        Ok(())
    }

    pub fn reset(&mut self) -> Result<()> {
        self.shift_register.reset(Level::Low)?;
        Ok(())
    }

    fn extract_row_data(&self, data: &u64, row: u8) -> u8 {
        /*
        MSBが(0,0)で、LSBが(7,7)
        最上列を取るには、63~56の範囲のビットが必要
        */
        let shift_amount = (7 - row) * 8;
        ((data >> shift_amount) & 0xFF) as u8
    }

    fn display_row(&mut self, row: u8, col_data: u8) -> Result<()> {
        let row_bits = 1u8 << (7 - row);
        let col_bits = !col_data; // 列データは0が点灯

        let control_data = ((col_bits as u16) << 8) | (row_bits as u16);

        self.shift_register.write(control_data)?;
        self.shift_register.latch()?;

        Ok(())
    }
}