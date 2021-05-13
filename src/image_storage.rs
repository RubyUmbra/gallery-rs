use std::path::{PathBuf, Path};
use std::fs;

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
    pub(crate) fn new(path: &Path) -> Result<ImageStorage, String> {
        let data: Vec<PathBuf> = fs::read_dir(path)
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|entry| entry.expect("Error").path())
            .filter(is_pic)
            .collect();

        if data.len() < 1 {
            Err("No images".to_string())
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

    pub(crate) fn remove(&mut self) {
        if !self.data.is_empty() {
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
