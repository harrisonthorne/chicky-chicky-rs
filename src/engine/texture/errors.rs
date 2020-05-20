use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct TextureError {
    error: Option<Box<dyn Error>>,
    detail: Option<String>,
}

impl TextureError {
    pub fn from_error<E: Error + 'static>(error: E) -> Self {
        Self {
            error: Some(Box::new(error)),
            detail: None,
        }
    }

    pub fn from_error_with_detail<E: Error + 'static>(error: E, detail: &str) -> Self {
        Self {
            error: Some(Box::new(error)),
            detail: Some(String::from(detail)),
        }
    }

    pub fn from_message(message: String) -> Self {
        Self {
            error: None,
            detail: Some(message),
        }
    }
}

impl fmt::Display for TextureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(error) = &self.error {
            if let Some(detail) = &self.detail {
                write!(f, "{}: {}", detail, error)
            } else {
                write!(f, "{}", error)
            }
        } else if let Some(detail) = &self.detail {
            write!(f, "{}", detail)
        } else {
            write!(f, "texture error, but no information supplied")
        }
    }
}

impl Error for TextureError {}
