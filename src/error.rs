use thiserror::Error;

#[derive(Error, Debug)]
pub enum MelottyError {
    #[error("Audio stream error: {0}")]
    Audio(#[from] rodio::StreamError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Decoder error: {0}")]
    Decoder(#[from] rodio::decoder::DecoderError),

    #[error("Play error: {0}")]
    Play(String),
}

impl From<rodio::PlayError> for MelottyError {
    fn from(err: rodio::PlayError) -> Self {
        MelottyError::Play(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, MelottyError>;
