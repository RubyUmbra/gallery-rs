use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Window(#[from] sdl2::video::WindowBuildError),
    #[error(transparent)]
    Sdl(#[from] sdl2::IntegerOrSdlError),
    #[error(transparent)]
    TextureValue(#[from] sdl2::render::TextureValueError),
    #[error(transparent)]
    Image(#[from] imagesize::ImageError),
    #[error("ERROR: {0}")]
    Other(String),
}

impl From<String> for AppError {
    fn from(value: String) -> Self {
        AppError::Other(value)
    }
}

impl From<&str> for AppError {
    fn from(value: &str) -> Self {
        AppError::Other(value.into())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
