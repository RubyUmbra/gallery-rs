use crate::errors::*;
use crate::event_message::EventMessage;
use crate::image_storage::ImageStorage;

use std::path::{Path, PathBuf};
use std::fs;

pub mod errors;
mod event_message;
mod gui;
mod image_storage;

pub fn run(path: &Path) -> Result<()> {
    let mut storage: ImageStorage = ImageStorage::new(path)?;

    let mut del = PathBuf::from(path);
    del.push("del");
    fs::create_dir_all(del.as_path())?;

    let mut gui_context = gui::GuiContext::new()?;

    gui_context.render_pic(storage.get().as_path())?;

    let mut event_pump = gui_context.sdl_context.event_pump()?;
    'mainloop: loop {
        for msg in event_pump.poll_iter().flat_map(EventMessage::try_from) {
            match msg {
                EventMessage::Quit => break 'mainloop,
                EventMessage::Next => storage.next(),
                EventMessage::Prev => storage.prev(),
                EventMessage::Move => storage.mv(del.as_path()),
            }
            if storage.is_empty() {
                return Err("No images".into());
            }
            gui_context.render_pic(storage.get().as_path())?;
        }
    }

    Ok(())
}
