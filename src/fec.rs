use lazy_static::lazy_static;
use reed_solomon::{Buffer, Decoder, Encoder};

use crate::error::{DroneCommError, Result};

lazy_static! {
    static ref FEC_ENCODER: Encoder = Encoder::new(223);
    static ref FEC_DECODER: Decoder = Decoder::new(223);
    static ref FEC_ENCODER_BUFFER: Buffer = Buffer::new(223);
    static ref FEC_DECODER_BUFFER: Buffer = Buffer::new(223);
}

pub fn encode_with_fec(data: &[u8]) -> Result<Vec<u8>> {
    let mut encoded_data = vec![0; FEC_ENCODER.rs_code_length()];
    FEC_ENCODER.encode(data, &mut encoded_data)?;
    Ok(encoded_data)
}
