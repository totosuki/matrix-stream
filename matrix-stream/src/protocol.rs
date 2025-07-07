use std::{
    io::{Error, ErrorKind, Result},
};

pub struct ProtocolParser;

impl ProtocolParser {
    pub fn validate_frame_data(data: &String) -> Result<()> {
        println!("[ProtocolParser] validate_frame_data: {}", data);

        if data.len() != 64 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "data length must be 64",
            ))
        }

        if !data.chars().all(|c| c == '0' || c == '1') {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "data content must be 0 or 1"
            ))
        }

        Ok(())
    }

    pub fn parse_frame_data(data: String) -> Option<u64> {
        println!("[ProtocolParser] parse_frame_data: {}", data);

        let parse_data = u64::from_str_radix(&data[..], 2);
        match parse_data {
            Ok(parse_data) => Some(parse_data),
            Err(e) => None
        }
    }
}