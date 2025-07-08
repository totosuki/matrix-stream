use std::{
    io::Result,
    time::{Duration, Instant}
};

use matrix_stream::drivers::osl641505::Osl641505;

fn main() -> Result<()> {
    println!("[display] start");

    let mut led = Osl641505::new(25, 24, 23, 10);
    let data: u64 = 0b10101010_01010101_10101010_01010101_10101010_01010101_10101010_01010101;
    let start = Instant::now();

    while start.elapsed() < Duration::from_secs(100) {
        led.draw(data)?;
    }

    led.reset()?;

    Ok(())
}
