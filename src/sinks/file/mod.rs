use std::{path::PathBuf, fs};

use super::Sink;

pub struct FileSink {
    path: PathBuf
}

impl FileSink {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            path: path.into()
        }
    }
}

impl Sink<String> for FileSink {
    fn sink(&self, data: String) {
        fs::write(&self.path, data).unwrap();
    }
}
