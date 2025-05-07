use crate::errors::*;
use crate::event_message::EventMessage;
use crate::image_storage::ImageStorage;
use std::fs;
use std::path::Path;

pub mod errors;
mod event_message;
mod gui;
mod image_storage;

pub fn run(path: &Path) -> Result<()> {
    let mut storage: ImageStorage = ImageStorage::new(path)?;

    let del: &Path = &path.join("del");
    fs::create_dir_all(del)?;

    let mut gui_context = gui::GuiContext::new()?;

    gui_context.render_pic(storage.get())?;

    let mut event_pump = gui_context.sdl_context.event_pump()?;
    'mainloop: loop {
        for msg in event_pump.poll_iter().flat_map(EventMessage::try_from) {
            match msg {
                EventMessage::Quit => break 'mainloop,
                EventMessage::Next => storage.next(),
                EventMessage::Prev => storage.prev(),
                EventMessage::Move => storage.mv(del),
            }
            if storage.is_empty() {
                return Err("No images".into());
            }
            gui_context.render_pic(storage.get())?;
        }
    }

    Ok(())
}
