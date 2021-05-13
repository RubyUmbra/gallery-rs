use std::{env, fs};
use std::path::{Path, PathBuf};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod gui;

fn is_pic(path: &PathBuf) -> bool {
    match path.extension() {
        Some(ext) => { ext == "png" || ext == "jpg" }
        None => { false }
    }
}

fn get_pics_paths(path: &Path) -> Result<Vec<PathBuf>, String> {
    let pics: Vec<PathBuf> = fs::read_dir(path)
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(|entry| entry.expect("TODO").path())
        .filter(is_pic)
        .collect();

    if pics.len() < 1 {
        Err("No images".to_string())
    } else {
        Ok(pics)
    }
}

pub fn run(path: &Path) -> Result<(), String> {
    let mut index = 0;
    let mut pics: Vec<PathBuf> = get_pics_paths(path)?;

    let mut gui_context = gui::GuiContext::new()?;

    gui_context.render_pic(pics[index].as_path())?;

    'mainloop: loop {
        for event in gui_context.sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Option::Some(Keycode::Escape), .. }
                | Event::KeyDown { keycode: Option::Some(Keycode::Q), .. }
                => break 'mainloop,
                Event::KeyDown { keycode: Option::Some(Keycode::Right), .. }
                => index = index + 1,
                Event::KeyDown { keycode: Option::Some(Keycode::Left), .. }
                => index = index + pics.len() - 1,
                Event::KeyDown { keycode: Option::Some(Keycode::Space), .. }
                => { pics.remove(index); }
                _ => {}
            }
            if pics.len() < 1 {
                return Err("No images".to_string());
            }
            index = index % pics.len();
            gui_context.render_pic(pics[index].as_path())?;
        }
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run /path/to/images")
    } else {
        run(Path::new(&args[1]))?;
    }

    Ok(())
}
