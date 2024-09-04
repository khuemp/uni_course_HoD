pub mod analytics;
pub mod channel;
pub mod pipeline;
pub use std::time::Instant;

use analytics::Style;
use channel::Channel;

pub fn parse_style_from_args() -> Style {
    std::env::args()
        .into_iter()
        .find(|arg| arg == "--csv")
        .map(|_| Style::CSV)
        .unwrap_or(Style::Pretty)
}

pub fn to_bits(byte: u8) -> Vec<u8> {
    (0..8).into_iter().map(|i| (byte >> i) & 0b01).collect()
}

pub fn to_byte(bits: &[u8]) -> u8 {
    (0..bits.len())
        .into_iter()
        .map(|i| bits[i] << i)
        .fold(0, |a, b| a | b)
}

pub fn channels() -> Vec<Channel> {
    vec![
        Channel::new(0.01, 0.01),
        Channel::new(0.1, 0.05),
        Channel::new(0.75, 0.1),
        Channel::new(0.5, 0.5),
        Channel::new(0.5, 0.75),
        Channel::new(0.75, 0.90),
    ]
}
