#[cfg(test)]
mod tests {
    use super::super::*;
    use chrono::Local;

    #[test]
    fn test_protocol_display() {
        assert_eq!(Protocol::TCP.to_string(), "TCP");
        assert_eq!(Protocol::UDP.to_string(), "UDP");
        assert_eq!(Protocol::ICMP.to_string(), "ICMP");
        assert_eq!(Protocol::ARP.to_string(), "ARP");
        assert_eq!(Protocol::Other("HTTP".to_string()).to_string(), "HTTP");
    }

    #[test]
    fn test_packet_creation() {
        let packet = Packet::new(
            Local::now(),
            "192.168.1.1".to_string(),
            "192.168.1.2".to_string(),
            Protocol::TCP,
            100,
            "Test packet".to_string(),
            vec![0, 1, 2, 3],
        );

        assert_eq!(packet.source, "192.168.1.1");
        assert_eq!(packet.destination, "192.168.1.2");
        assert_eq!(packet.protocol, Protocol::TCP);
        assert_eq!(packet.length, 100);
        assert_eq!(packet.info, "Test packet");
        assert_eq!(packet.raw_data, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_packet_time_str() {
        let packet = Packet::new(
            Local::now(),
            "192.168.1.1".to_string(),
            "192.168.1.2".to_string(),
            Protocol::TCP,
            100,
            "Test packet".to_string(),
            vec![],
        );

        let time_str = packet.time_str();
        assert!(time_str.len() > 0);
        assert!(time_str.contains(":"));
    }
}