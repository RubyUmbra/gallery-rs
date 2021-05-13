use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::rect::Rect;
use sdl2::video::{WindowContext};
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::{Sdl};
use std::path::Path;

pub(crate) struct GuiContext {
    pub(crate) sdl_context: Sdl,
    display_bounds: Rect,
    canvas: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
}

impl GuiContext {
    pub(crate) fn new() -> Result<GuiContext, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        match video_subsystem.num_video_displays() {
            Ok(num) => { if num < 1 { return Err("No displays".to_string()); } }
            Err(_) => {}
        }
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
        let display_bounds = video_subsystem.display_bounds(0)?;
        let window = video_subsystem
            .window("Gallery", display_bounds.width(), display_bounds.height())
            .fullscreen()
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window
            .into_canvas()
            .software()
            .build()
            .map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();

        Ok(
            GuiContext {
                sdl_context,
                display_bounds,
                canvas,
                texture_creator,
            }
        )
    }

    pub(crate) fn render_pic(&mut self, path: &Path) -> Result<(), String> {
        let texture = self.texture_creator.load_texture(path)?;
        let img_sz = imagesize::size(path)
            .map_err(|e| e.to_string())?;
        let mut dst = Rect::new(0, 0, img_sz.width as u32, img_sz.height as u32);
        if self.display_bounds.width() < img_sz.width as u32 {
            let coeff = self.display_bounds.width() as f32 / img_sz.width as f32;
            dst = Rect::new(0, 0, (img_sz.width as f32 * coeff) as u32, (img_sz.height as f32 * coeff) as u32);
        }
        if self.display_bounds.height() < img_sz.height as u32 {
            let coeff = self.display_bounds.height() as f32 / img_sz.height as f32;
            dst = Rect::new(0, 0, (img_sz.width as f32 * coeff) as u32, (img_sz.height as f32 * coeff) as u32);
        }
        self.canvas.clear();
        self.canvas.copy(&texture, None, dst)?;
        self.canvas.present();

        Ok(())
    }
}
