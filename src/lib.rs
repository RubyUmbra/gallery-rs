use crate::image_storage::ImageStorage;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::path::{Path, PathBuf};
use std::{env, fs};

mod gui;
mod image_storage;

pub fn run() -> Result<(), String> {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("path")
                .about("path of directory with pictures to sort")
                .index(1)
                .required(true),
        );

    let matches = app.get_matches();

    let path = matches
        .value_of_t::<PathBuf>("path")
        .or(env::current_dir())
        .map_err(|e| e.to_string())?;

    run_internal(path.as_path())
}

fn run_internal(path: &Path) -> Result<(), String> {
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
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape),
                    ..
                }
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Q),
                    ..
                } => break 'mainloop,
                Event::KeyDown {
                    keycode: Option::Some(Keycode::Right),
                    ..
                } => storage.next(),
                Event::KeyDown {
                    keycode: Option::Some(Keycode::Left),
                    ..
                } => storage.prev(),
                Event::KeyDown {
                    keycode: Option::Some(Keycode::Space),
                    ..
                } => storage.mv(del.as_path()),
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
