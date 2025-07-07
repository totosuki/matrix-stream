#[cfg(feature = "raspi")]
pub mod drivers;
pub mod display_controller;
pub mod protocol;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Low,
    High,
}