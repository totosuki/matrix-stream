use std::{
    io::Result,
    thread,
    time::Duration,
};

use rppal::gpio::{Gpio, OutputPin};

use crate::Level;

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

    pub fn latch(&mut self) -> Result<()> {
        Self::positive_edge(&mut self.rclk)?;
        Ok(())
    }

    pub fn reset(&mut self, level: Level) -> Result<()> {
        let bit = if level == Level::High {1} else {0};

        for _ in 0..16 {
            self.write(bit)?;
        }

        self.ser.set_low();
        self.rclk.set_low();
        self.srclk.set_low();

        Ok(())
    }

    pub fn write(&mut self, data: u16) -> Result<()> {
        for i in 0..16 {
            let bit = (data >> i) & 1;
            self.shift(if bit == 1 {Level::High} else {Level::Low})?;
        }
        Ok(())
    }

    fn get_outputpin(num: u8, gpio: &Gpio) -> OutputPin {
        let pin = gpio.get(num).unwrap();
        pin.into_output()
    }

    fn positive_edge(pin: &mut OutputPin) -> Result<()> {
        pin.set_high();
        thread::sleep(Duration::from_micros(1));
        pin.set_low();
        Ok(())
    }

    fn shift(&mut self, level: Level) -> Result<()> {
        if level == Level::High {
            self.ser.set_high();
        }
        else {
            self.ser.set_low();
        }
        Self::positive_edge(&mut self.srclk)?;
        Ok(())
    }
}