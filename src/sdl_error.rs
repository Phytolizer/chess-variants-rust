use crate::Error;

pub(crate) trait ToSdl<T> {
    fn sdl_error(self) -> Result<T, Error>;
}

impl<T> ToSdl<T> for Result<T, String> {
    fn sdl_error(self) -> Result<T, Error> {
        self.map_err(|e| Error::Sdl(SdlError::Other(e)))
    }
}

impl<T> ToSdl<T> for Result<T, sdl2::render::TargetRenderError> {
    fn sdl_error(self) -> Result<T, Error> {
        self.map_err(|e| Error::Sdl(e.into()))
    }
}

impl<T> ToSdl<T> for Result<T, sdl2::video::WindowBuildError> {
    fn sdl_error(self) -> Result<T, Error> {
        self.map_err(|e| Error::Sdl(e.into()))
    }
}

impl<T> ToSdl<T> for Result<T, sdl2::render::TextureValueError> {
    fn sdl_error(self) -> Result<T, Error> {
        self.map_err(|e| Error::Sdl(e.into()))
    }
}

impl<T> ToSdl<T> for Result<T, sdl2::IntegerOrSdlError> {
    fn sdl_error(self) -> Result<T, Error> {
        self.map_err(|e| Error::Sdl(e.into()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SdlError {
    #[error("{0}")]
    Other(String),

    #[error(transparent)]
    TargetRender(#[from] sdl2::render::TargetRenderError),

    #[error("building window: {0}")]
    WindowBuild(#[from] sdl2::video::WindowBuildError),

    #[error(transparent)]
    TextureValue(#[from] sdl2::render::TextureValueError),

    #[error(transparent)]
    IntegerOrSdl(#[from] sdl2::IntegerOrSdlError),
}
