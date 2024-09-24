use thiserror::Error;

#[derive(Error, Debug)]
pub enum DroneCommError {
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    #[error("Decryption error: {0}")]
    DecryptionError(String),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] bincode::Error),
    #[error("System time error: {0}")]
    SystemTimeError(#[from] std::time::SystemTimeError),
    #[error("SDR error: {0}")]
    SdrError(String),
}

pub type Result<T> = std::result::Result<T, DroneCommError>;
