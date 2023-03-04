use thiserror::Error;

#[derive(Error, Debug)]
pub enum MinesweeperError {
    #[error("Gif Encoding Error")]
    GifEncoding,
    #[error("Image insertion Error")]
    ImageInsertion,
    #[error("Image Save Error")]
    ImageSave,
}
