use aes_gcm::{Aes256Gcm, NewAead};

use crate::crypto::{decrypt_payload, encrypt_payload, generate_encryption_key, generate_nonce};
use crate::error::Result;
use crate::fec::encode_with_fec;
use crate::fhss::FrequencyHopper;
use crate::packet::Packet;

pub struct DroneComm {
    frequency_hopper: FrequencyHopper,
    cipher: Aes256Gcm,
    current_nonce: [u8; 12],
    nonce_counter: u64,
    payload_buffer: Vec<u8>,
    packet_buffer: Vec<u8>,
}

impl DroneComm {
    pub fn new() -> Result<Self> {
        let encryption_key = generate_encryption_key();
        let cipher = Aes256Gcm::new(aes_gcm::Key::from_slice(&encryption_key));

        Ok(DroneComm {
            frequency_hopper: FrequencyHopper::new(2.4e9, 2.5e9, 100),
            cipher,
            current_nonce: generate_nonce(),
            nonce_counter: 0,
            payload_buffer: Vec::with_capacity(1024),
            packet_buffer: Vec::with_capacity(2048),
        })
    }

    fn update_nonce(&mut self) {
        self.nonce_counter += 1;
        if self.nonce_counter % 1000 == 0 {
            self.current_nonce = generate_nonce();
            self.nonce_counter = 0;
        } else {
            let counter_bytes = self.nonce_counter.to_be_bytes();
            self.current_nonce[4..].copy_from_slice(&counter_bytes);
        }
    }

    pub fn send_message(&mut self, payload: &[u8]) -> Result<()> {
        self.payload_buffer.clear();
        self.payload_buffer.extend_from_slice(payload);

        let encrypted = encrypt_payload(&self.cipher, &self.current_nonce, &self.payload_buffer)?;
        self.update_nonce();

        let packet = Packet::new(encrypted)?;

        self.packet_buffer.clear();
        bincode::serialize_into(&mut self.packet_buffer, &packet)?;
        let encoded = encode_with_fec(&self.packet_buffer)?;

        let frequency = self.frequency_hopper.next_frequency();
        println!("Sending message on frequency: {} Hz", frequency);

        // TODO: Implement actual SDR transmission here
        // If transmission is successful:
        self.frequency_hopper
            .report_success(self.frequency_hopper.current_index);
        // If interference is detected:
        // self.frequency_hopper.report_interference(self.frequency_hopper.current_index);

        Ok(())
    }

    pub fn receive_message(&mut self) -> Result<Vec<u8>> {
        let frequency = self.frequency_hopper.next_frequency();
        println!("Listening for message on frequency: {} Hz", frequency);

        // TODO: Implement actual SDR reception here
        // If reception is successful:
        self.frequency_hopper
            .report_success(self.frequency_hopper.current_index);
        // If interference is detected:
        // self.frequency_hopper.report_interference(self.frequency_hopper.current_index);

        let received_encrypted: Vec<u8> = vec![]; // This should be filled with actual received data

        let decrypted = decrypt_payload(&self.cipher, &self.current_nonce, &received_encrypted)?;
        self.update_nonce();
        Ok(decrypted)
    }
}
