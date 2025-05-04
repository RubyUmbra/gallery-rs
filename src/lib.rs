use crate::errors::*;
use crate::image_storage::ImageStorage;

use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, Command};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::path::{Path, PathBuf};
use std::{env, fs};

pub mod errors;
mod gui;
mod image_storage;

pub fn run() -> Result<()> {
    let app = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("path")
                .help("path of directory with pictures to sort")
                .index(1)
                .required(true)
                .value_parser(clap::value_parser!(PathBuf)),
        );

    let matches = app.get_matches();

    let path = matches
        .get_one::<PathBuf>("path")
        .cloned()
        .ok_or("")
        .or(env::current_dir())?;

    run_internal(path.as_path())
}

fn run_internal(path: &Path) -> Result<()> {
    let mut storage: ImageStorage = ImageStorage::new(path)?;

    let mut del = PathBuf::from(path);
    del.push("del");
    fs::create_dir_all(del.as_path())?;

    let mut gui_context = gui::GuiContext::new()?;

    gui_context.render_pic(storage.get().as_path())?;

    'mainloop: loop {
        for event in gui_context.sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'mainloop,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => storage.next(),
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => storage.prev(),
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => storage.mv(del.as_path()),
                _ => {}
            }
            if storage.is_empty() {
                return Err("No images".into());
            }
            gui_context.render_pic(storage.get().as_path())?;
        }
    }

    Ok(())
}
