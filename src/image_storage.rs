use crate::errors::*;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

fn is_pic(path: &PathBuf) -> bool {
    path.extension()
        .map(|ext| ext == "png" || ext == "jpg")
        .unwrap_or(false)
}

pub(crate) struct ImageStorage {
    data: Vec<PathBuf>,
    current: usize,
}

impl ImageStorage {
    pub(crate) fn new(path: &Path) -> Result<ImageStorage> {
        let mut data: Vec<PathBuf> = fs::read_dir(path)?
            .into_iter()
            .filter_map(|res| res.map(|entry| entry.path()).ok())
            .filter(is_pic)
            .collect();

        data.sort_by_key(|path| {
            path.file_name()
                .and_then(OsStr::to_str)
                .unwrap()
                .to_lowercase()
        });

        if data.is_empty() {
            Err("No images".into())
        } else {
            Ok(ImageStorage { data, current: 0 })
        }
    }

    pub(crate) fn next(&mut self) {
        if !self.data.is_empty() {
            self.current = (self.current + 1) % self.data.len()
        }
    }

    pub(crate) fn prev(&mut self) {
        if !self.data.is_empty() {
            self.current = (self.data.len() + self.current - 1) % self.data.len()
        }
    }

    pub(crate) fn get(&self) -> &PathBuf {
        &self.data[self.current]
    }

    pub(crate) fn mv(&mut self, path: &Path) {
        if !self.data.is_empty() {
            let mut new = path.to_path_buf();
            new.push(self.get().file_name().expect("Error"));
            fs::rename(self.get().as_path(), new).expect("Error");
            self.data.remove(self.current);
            if !self.data.is_empty() {
                self.current %= self.data.len();
            }
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
