use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Packet {
    sequence_number: u32,
    timestamp: u64,
    payload: Vec<u8>,
    checksum: u32,
}

impl Packet {
    pub fn new(payload: Vec<u8>) -> Result<Self> {
        let sequence_number = rand::random();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        let mut packet = Packet {
            sequence_number,
            timestamp,
            payload,
            checksum: 0,
        };

        packet.checksum = packet.calculate_checksum();
        Ok(packet)
    }

    fn calculate_checksum(&self) -> u32 {
        let mut sum = 0u32;
        sum = sum.wrapping_add(self.sequence_number);
        sum = sum.wrapping_add(self.timestamp as u32);
        for byte in &self.payload {
            sum = sum.wrapping_add(*byte as u32);
        }
        sum
    }
}
