use std::{env, fs};
use std::path::{Path, PathBuf};
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use imagesize;
use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::video::WindowContext;

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

fn render_pic(
    canvas: &mut WindowCanvas,
    path: &Path,
    texture_creator: &TextureCreator<WindowContext>,
    display_bounds: Rect,
) -> Result<(), String> {
    let texture = texture_creator.load_texture(path)?;
    let img_sz = imagesize::size(path)
        .map_err(|e| e.to_string())?;
    let mut dst = Rect::new(0, 0, img_sz.width as u32, img_sz.height as u32);
    if display_bounds.width() < img_sz.width as u32 {
        let coeff = display_bounds.width() as f32 / img_sz.width as f32;
        dst = Rect::new(0, 0, (img_sz.width as f32 * coeff) as u32, (img_sz.height as f32 * coeff) as u32);
    }
    if display_bounds.height() < img_sz.height as u32 {
        let coeff = display_bounds.height() as f32 / img_sz.height as f32;
        dst = Rect::new(0, 0, (img_sz.width as f32 * coeff) as u32, (img_sz.height as f32 * coeff) as u32);
    }
    canvas.clear();
    canvas.copy(&texture, None, dst)?;
    canvas.present();

    Ok(())
}

pub fn run(path: &Path) -> Result<(), String> {
    let mut index = 0;
    let pics: Vec<PathBuf> = get_pics_paths(path)?;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    match video_subsystem.num_video_displays() {
        Ok(num) => {
            if num < 1 {
                return Err("No displays".to_string());
            }
        }
        Err(_) => {}
    }
    let display_bounds = video_subsystem.display_bounds(0)?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem
        .window("Gallery", display_bounds.width(), display_bounds.height())
        .fullscreen()
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    render_pic(&mut canvas, pics[index].as_path(), &texture_creator, display_bounds)?;

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
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
                } => {
                    index = (index + 1) % pics.len();
                    render_pic(&mut canvas, pics[index].as_path(), &texture_creator, display_bounds)?;
                }
                Event::KeyDown {
                    keycode: Option::Some(Keycode::Left),
                    ..
                } => {
                    index = (index + pics.len() - 1) % pics.len();
                    render_pic(&mut canvas, pics[index].as_path(), &texture_creator, display_bounds)?;
                }
                _ => {}
            }
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
