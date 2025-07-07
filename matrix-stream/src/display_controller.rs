use std::{
    io::{Error, ErrorKind, Result}
};

// use crate::drivers::osl641505::Osl641505;
use crate::protocol::ProtocolParser;

pub struct DisplayController;
// pub struct DisplayController {
//     led_matrix: Osl641505,
// }

impl DisplayController {
    // pub fn new(data_pin: u8, latch_pin: u8, clock_pin: u8, duration: u64) -> Self {
    //     Self {
    //         led_matrix: Osl641505::new(25, 24, 23, 100);
    //     }
    // }

    pub fn display(data: String) -> Result<()> {
        ProtocolParser::validate_frame_data(&data)?;

        let data = ProtocolParser::parse_frame_data(data);
        // let mut led_matrix = Osl641505::new(25, 24, 23, 100);
        
        match data {
            Some(data) => {
                println!("[DisplayController] display data: {}", data);
            },
            None => {
                println!("[DisplayController] can not parse data");
            },
        };

        Ok(())
    }
}