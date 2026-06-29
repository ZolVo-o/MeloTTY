use thiserror::Error;

#[derive(Error, Debug)]
pub enum TermusicError {
    #[error("Audio stream error: {0}")]
    Audio(#[from] rodio::StreamError),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Decoder error: {0}")]
    Decoder(#[from] rodio::decoder::DecoderError),
    
    #[error("Play error: {0}")]
    Play(String),
    
    #[error("Playlist error: {0}")]
    Playlist(String),
    
    #[error("Terminal error: {0}")]
    Terminal(String),
}

impl From<rodio::PlayError> for TermusicError {
    fn from(err: rodio::PlayError) -> Self {
        TermusicError::Play(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, TermusicError>;
