use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use anyhow::*;
use image::{load_from_memory, GenericImage, RgbaImage};

/// A shared image
#[derive(Clone)]
pub struct Texture {
    data: Arc<Mutex<Inner>>,
    label: Arc<str>,
    width: u32,
    height: u32,
}

enum Inner {
    Defined { path: PathBuf },
    Decoded { buffer: RgbaImage },
}

impl Texture {
    pub(crate) fn from_bytes(data: &[u8], label: impl Into<Arc<str>>) -> Result<Self> {
        let buffer = load_from_memory(data)?.to_rgba8();
        let width = buffer.width();
        let height = buffer.height();
        Ok(Texture {
            data: Arc::new(Mutex::new(Inner::Decoded { buffer })),
            label: label.into(),
            width,
            height,
        })
    }

    pub(crate) fn from_path(path: PathBuf) -> Self {
        let label = format!("{}", path.display()).into();
        Texture {
            data: Arc::new(Mutex::new(Inner::Defined { path })),
            label,
            width: 0,
            height: 0,
        }
    }

    pub(crate) fn resize(&self, width: u32, height: u32) -> Result<Self> {
        if width != self.width && height != self.height {
            let data = self.data.lock().unwrap();
            match &*data {
                Inner::Defined { path } => Ok(Texture {
                    data: Arc::new(Mutex::new(Inner::Defined { path: path.clone() })),
                    label: format!("{}#{}x{}", self.label, width, height).into(),
                    width,
                    height,
                }),
                Inner::Decoded { buffer } => {
                    let mut new_image: RgbaImage = RgbaImage::new(width, height);
                    new_image.copy_from(buffer, 0, 0)?;
                    Ok(Texture {
                        data: Arc::new(Mutex::new(Inner::Decoded { buffer: new_image })),
                        label: format!("{}#{}x{}", self.label, width, height).into(),
                        width,
                        height,
                    })
                }
            }
        } else {
            Ok(self.clone())
        }
    }

    pub(crate) fn width(&self) -> u32 {
        self.width
    }

    pub(crate) fn height(&self) -> u32 {
        self.height
    }
}
