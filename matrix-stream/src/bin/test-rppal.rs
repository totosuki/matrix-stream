use std::{
    io::{Result},
    time,
    thread,
};

use rppal::{gpio};

fn main() -> Result<()> {
    let gpio = gpio::Gpio::new().unwrap();
    let pin = gpio.get(25).unwrap();
    let mut pin = pin.into_output();

    let five_sec = time::Duration::from_secs(5);

    pin.set_high();

    thread::sleep(five_sec);

    pin.set_low();

    Ok(())
}