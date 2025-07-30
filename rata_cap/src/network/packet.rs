use chrono::{DateTime, Local};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Packet {
    pub timestamp: DateTime<Local>,
    pub source: String,
    pub destination: String,
    pub protocol: Protocol,
    pub length: usize,
    pub info: String,
    pub raw_data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Protocol {
    TCP,
    UDP,
    ICMP,
    ARP,
    Other(String),
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Protocol::TCP => write!(f, "TCP"),
            Protocol::UDP => write!(f, "UDP"),
            Protocol::ICMP => write!(f, "ICMP"),
            Protocol::ARP => write!(f, "ARP"),
            Protocol::Other(name) => write!(f, "{}", name),
        }
    }
}

impl Packet {
    pub fn new(
        timestamp: DateTime<Local>,
        source: String,
        destination: String,
        protocol: Protocol,
        length: usize,
        info: String,
        raw_data: Vec<u8>,
    ) -> Self {
        Self {
            timestamp,
            source,
            destination,
            protocol,
            length,
            info,
            raw_data,
        }
    }

    pub fn time_str(&self) -> String {
        self.timestamp.format("%H:%M:%S%.3f").to_string()
    }
}