use std::{
    io::Result,
    time::{Duration, Instant},
    thread,
};

use rppal::gpio::{Gpio, OutputPin};

fn main() -> Result<()> {
    let gpio = Gpio::new().unwrap();
    let mut ser = get_outputpin(25, &gpio);
    let mut rclk = get_outputpin(24, &gpio);
    let mut srclk = get_outputpin(23, &gpio);
    let duration = Duration::from_secs(10);
    let data = 0b10111111_01000000;

    let start = Instant::now();
    while start.elapsed() < duration {
        for i in 0..16 {
            write(data >> i & 1, &mut ser, &mut srclk);
        }

        positive_edge(&mut rclk);
        thread::sleep(Duration::from_millis(100));
    }

    reset(&mut ser, &mut srclk, &mut rclk);

    Ok(())
}

fn get_outputpin(num: u8, gpio: &Gpio) -> OutputPin {
    let pin = gpio.get(num).unwrap();
    pin.into_output()
}

fn reset(ser: &mut OutputPin, srclk: &mut OutputPin, rclk: &mut OutputPin) {
    let data = 0b00000000_00000000;
    for i in 0..16 {
        write(data >> i & 1, ser, srclk);
    }
    positive_edge(rclk);
    ser.set_low();
    srclk.set_low();
    rclk.set_low();
}

fn write(bit: i32, ser: &mut OutputPin, srclk: &mut OutputPin) {
    if bit == 1 {
        ser.set_high();
    }
    else {
        ser.set_low();
    }
    positive_edge(srclk);
}

fn positive_edge(pin: &mut OutputPin) {
    pin.set_high();
    pin.set_low();
}