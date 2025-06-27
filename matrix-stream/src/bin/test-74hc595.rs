use std::{
    io::{Result},
    time,
    thread,
};

use rppal::gpio::{Gpio, OutputPin};

fn main() -> Result<()> {
    let mut ser = get_outputpin(25);
    let mut rclk = get_outputpin(24);
    let mut srclk = get_outputpin(23);
    let five_sec = time::Duration::from_secs(5);
    let data = [1, 1, 1, 1, 1, 1, 1, 1];

    for i in data {
        write(i, &mut ser, &mut srclk);
    }

    positive_edge(&mut rclk);
    thread::sleep(five_sec);

    reset(&mut ser, &mut srclk, &mut rclk);

    Ok(())
}

fn get_outputpin(num: u8) -> OutputPin {
    let gpio = Gpio::new().unwrap();
    let pin = gpio.get(num).unwrap();
    pin.into_output()
}

fn reset(ser: &mut OutputPin, srclk: &mut OutputPin, rclk: &mut OutputPin) {
    ser.set_low();
    for _ in 0..8 {
        positive_edge(srclk);
    }
    positive_edge(rclk);
}

fn write(bit: i32, ser: &mut OutputPin, srclk: &mut OutputPin) {
    if bit == 1 {
        ser.set_high();
    }
    positive_edge(srclk);
    ser.set_low();
}

fn positive_edge(pin: &mut OutputPin) {
    pin.set_high();
    pin.set_low();
}