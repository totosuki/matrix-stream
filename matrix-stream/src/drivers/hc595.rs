use std::{
    io::Result, process::Output, thread, time::{Duration, Instant}
};

use rppal::gpio::{Gpio, OutputPin};

pub struct Hc595 {
    ser: OutputPin,
    rclk: OutputPin,
    srclk: OutputPin,
}

impl Hc595 {
    pub fn new(ser_num: u8, rclk_num: u8, srclk: u8) -> Self {
        let gpio = Gpio::new().unwrap();
        Self {
            ser: Self::get_outputpin(ser_num, &gpio),
            rclk: Self::get_outputpin(rclk_num, &gpio),
            srclk: Self::get_outputpin(srclk, &gpio),
        }
    }

    pub fn write(&mut self, bit: u8) -> Result<()> {
        if bit == 1 {
            self.ser.set_high();
        }
        else {
            self.ser.set_low();
        }
        Self::positive_edge(&mut self.srclk)?;
        Ok(())
    }

    pub fn latch(&mut self) -> Result<()> {
        Self::positive_edge(&mut self.rclk)?;
        Ok(())
    }

    pub fn reset(&mut self, is_low: bool) -> Result<()> {
        let bit = if is_low { 0 } else { 1 };

        for _ in 0..16 {
            self.write(bit)?;
        }

        self.ser.set_low();
        self.rclk.set_low();
        self.srclk.set_low();

        Ok(())
    }

    fn get_outputpin(num: u8, gpio: &Gpio) -> OutputPin {
        let pin = gpio.get(num).unwrap();
        pin.into_output()
    }

    fn positive_edge(pin: &mut OutputPin) -> Result<()> {
        pin.set_high();
        thread::sleep(Duration::from_millis(1));
        pin.set_low();
        Ok(())
    }
}