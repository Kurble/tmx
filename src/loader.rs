use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use anyhow::*;

use crate::Map;

#[derive(Clone)]
pub(crate) struct TmxLoadContext<'a> {
    relative: Arc<Path>,
    lifetime: &'a (),
}

impl<'a> TmxLoadContext<'a> {
    pub fn load_file<'p>(&'p self, path: impl AsRef<Path> + Send + 'p) -> Result<Vec<u8>> {
        Ok(std::fs::read(self.file_path(path))?)
    }

    pub fn file_path(&self, path: impl AsRef<Path>) -> PathBuf {
        let mut joined = PathBuf::new();
        for c in self.relative.join(path.as_ref()).components() {
            match c {
                Component::Prefix(prefix) => joined.push(prefix.as_os_str()),
                Component::RootDir => joined.push("/"),
                Component::CurDir => (),
                Component::ParentDir => {
                    joined.pop();
                }
                Component::Normal(c) => joined.push(c),
            }
        }
        joined
    }

    pub fn file_directory(&self, path: impl AsRef<Path>) -> Self {
        Self {
            relative: if let Some(parent) = path.as_ref().parent() {
                Arc::from(self.relative.join(parent))
            } else {
                self.relative.clone()
            },
            lifetime: self.lifetime,
        }
    }
}

/// Load Map from a file.
pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Map> {
    let path = path.as_ref();
    let context = ();
    let context = if let Some(parent) = path.parent() {
        TmxLoadContext {
            relative: Arc::from(parent.to_path_buf()),
            lifetime: &context,
        }
    } else {
        TmxLoadContext {
            relative: Path::new(".").to_path_buf().into(),
            lifetime: &context,
        }
    };

    let reader = xml::EventReader::new(std::fs::File::open(path)?);

    Map::load_from_xml_reader(context, reader)
}
