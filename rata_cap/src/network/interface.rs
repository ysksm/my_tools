use anyhow::Result;
use pcap::{Device, Error as PcapError};

#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub description: Option<String>,
    pub addresses: Vec<String>,
    pub is_loopback: bool,
}

impl NetworkInterface {
    pub fn list_interfaces() -> Result<Vec<NetworkInterface>> {
        let devices = Device::list().map_err(|e| match e {
            PcapError::PcapError(msg) => anyhow::anyhow!("PCAP error: {}", msg),
            _ => anyhow::anyhow!("Failed to list network devices"),
        })?;

        let interfaces = devices
            .into_iter()
            .map(|device| {
                let addresses: Vec<String> = device
                    .addresses
                    .iter()
                    .filter_map(|addr| {
                        if let std::net::IpAddr::V4(ipv4) = addr.addr {
                            Some(ipv4.to_string())
                        } else if let std::net::IpAddr::V6(ipv6) = addr.addr {
                            Some(ipv6.to_string())
                        } else {
                            None
                        }
                    })
                    .collect();

                NetworkInterface {
                    name: device.name.clone(),
                    description: device.desc,
                    addresses,
                    is_loopback: device.flags.is_loopback(),
                }
            })
            .collect();

        Ok(interfaces)
    }

    pub fn display_name(&self) -> String {
        if let Some(desc) = &self.description {
            format!("{} ({})", self.name, desc)
        } else {
            self.name.clone()
        }
    }
}