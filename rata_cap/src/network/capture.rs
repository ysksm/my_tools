use anyhow::Result;
use chrono::Local;
use pcap::{Capture, Device, Packet as PcapPacket};
use tokio::sync::mpsc;

use super::packet::{Packet, Protocol};

pub struct PacketCapture {
    device_name: String,
}

impl PacketCapture {
    pub fn new(device_name: String) -> Self {
        Self { device_name }
    }

    pub async fn start_capture(self, tx: mpsc::Sender<Packet>) -> Result<()> {
        let device = Device::list()?
            .into_iter()
            .find(|d| d.name == self.device_name)
            .ok_or_else(|| anyhow::anyhow!("Device not found: {}", self.device_name))?;

        let mut cap = Capture::from_device(device)?
            .promisc(true)
            .snaplen(65535)
            .timeout(1000)
            .open()
            .map_err(|e| anyhow::anyhow!("Failed to open capture device: {}. Make sure you have proper permissions.", e))?;

        tokio::task::spawn_blocking(move || -> Result<()> {
            loop {
                match cap.next_packet() {
                    Ok(packet) => {
                        if let Some(parsed) = parse_packet(&packet) {
                            let _ = tx.blocking_send(parsed);
                        }
                    }
                    Err(pcap::Error::TimeoutExpired) => continue,
                    Err(e) => return Err(anyhow::anyhow!("Capture error: {}", e)),
                }
            }
        })
        .await?
    }
}

fn parse_packet(pcap_packet: &PcapPacket) -> Option<Packet> {
    let data = pcap_packet.data;
    if data.len() < 14 {
        return None;
    }

    let eth_type = u16::from_be_bytes([data[12], data[13]]);
    
    match eth_type {
        0x0800 => parse_ipv4_packet(data),
        0x0806 => parse_arp_packet(data),
        _ => {
            Some(Packet::new(
                Local::now(),
                "Unknown".to_string(),
                "Unknown".to_string(),
                Protocol::Other(format!("0x{:04x}", eth_type)),
                data.len(),
                "Unknown packet type".to_string(),
                data.to_vec(),
            ))
        }
    }
}

fn parse_ipv4_packet(data: &[u8]) -> Option<Packet> {
    if data.len() < 34 {
        return None;
    }

    let ip_header_start = 14;
    let protocol = data[ip_header_start + 9];
    
    let src_ip = format!(
        "{}.{}.{}.{}",
        data[ip_header_start + 12],
        data[ip_header_start + 13],
        data[ip_header_start + 14],
        data[ip_header_start + 15]
    );
    
    let dst_ip = format!(
        "{}.{}.{}.{}",
        data[ip_header_start + 16],
        data[ip_header_start + 17],
        data[ip_header_start + 18],
        data[ip_header_start + 19]
    );

    let (protocol_type, info) = match protocol {
        1 => (Protocol::ICMP, "ICMP packet".to_string()),
        6 => {
            let src_port = u16::from_be_bytes([data[34], data[35]]);
            let dst_port = u16::from_be_bytes([data[36], data[37]]);
            (Protocol::TCP, format!("{}→{}", src_port, dst_port))
        }
        17 => {
            let src_port = u16::from_be_bytes([data[34], data[35]]);
            let dst_port = u16::from_be_bytes([data[36], data[37]]);
            (Protocol::UDP, format!("{}→{}", src_port, dst_port))
        }
        _ => (Protocol::Other(format!("IP:{}", protocol)), "".to_string()),
    };

    Some(Packet::new(
        Local::now(),
        src_ip,
        dst_ip,
        protocol_type,
        data.len(),
        info,
        data.to_vec(),
    ))
}

fn parse_arp_packet(data: &[u8]) -> Option<Packet> {
    if data.len() < 42 {
        return None;
    }

    let operation = u16::from_be_bytes([data[20], data[21]]);
    let info = match operation {
        1 => "ARP Request",
        2 => "ARP Reply",
        _ => "ARP Unknown",
    };

    let src_ip = format!("{}.{}.{}.{}", data[28], data[29], data[30], data[31]);
    let dst_ip = format!("{}.{}.{}.{}", data[38], data[39], data[40], data[41]);

    Some(Packet::new(
        Local::now(),
        src_ip,
        dst_ip,
        Protocol::ARP,
        data.len(),
        info.to_string(),
        data.to_vec(),
    ))
}