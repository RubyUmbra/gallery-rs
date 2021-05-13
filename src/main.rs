use std::{env, fs};
use std::path::{Path, PathBuf};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::image_storage::ImageStorage;

mod gui;
mod image_storage;

pub fn run(path: &Path) -> Result<(), String> {
    let mut storage: ImageStorage = ImageStorage::new(path)?;

    let mut del = PathBuf::from(path);
    del.push("del");
    fs::create_dir_all(del.as_path()).map_err(|e| e.to_string())?;

    let mut gui_context = gui::GuiContext::new()?;

    gui_context.render_pic(storage.get().as_path())?;

    'mainloop: loop {
        for event in gui_context.sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Option::Some(Keycode::Escape), .. }
                | Event::KeyDown { keycode: Option::Some(Keycode::Q), .. }
                => break 'mainloop,
                Event::KeyDown { keycode: Option::Some(Keycode::Right), .. }
                => storage.next(),
                Event::KeyDown { keycode: Option::Some(Keycode::Left), .. }
                => storage.prev(),
                Event::KeyDown { keycode: Option::Some(Keycode::Space), .. }
                => storage.mv(del.as_path()),
                _ => {}
            }
            if storage.is_empty() {
                return Err("No images".to_string());
            }
            gui_context.render_pic(storage.get().as_path())?;
        }
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();

    let path = if args.len() < 2 {
        env::current_dir().map_err(|e| e.to_string())?
    } else {
        Path::new(&args[1]).to_path_buf()
    };

    run(path.as_path())
}
