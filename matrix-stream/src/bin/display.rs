use std::{
    io::Result,
    time::{Duration, Instant}
};

use matrix_stream::drivers::osl641505::Osl641505;

fn main() -> Result<()> {
    println!("[display] start");

    let mut led = Osl641505::new(25, 24, 23,100);
    let data = 0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111;

    let start = Instant::now();

    while start.elapsed() < Duration::from_secs(100) {
        led.draw(&data)?;
    }

    led.reset()?;

    Ok(())
}
