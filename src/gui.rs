use crate::errors::*;
use image::RgbaImage;
use sdl2::Sdl;
use sdl2::image::InitFlag;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use std::path::Path;

pub(crate) struct GuiContext {
    pub(crate) sdl_context: Sdl,
    canvas: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
}

fn resize_image(img: RgbaImage, window_width: u32, window_height: u32) -> Result<RgbaImage> {
    let (width, height) = img.dimensions();
    let width_ratio = window_width as f32 / width as f32;
    let height_ratio = window_height as f32 / height as f32;
    let scale_factor = width_ratio.min(height_ratio);
    Ok(image::imageops::resize(
        &img,
        (width as f32 * scale_factor) as u32,
        (height as f32 * scale_factor) as u32,
        image::imageops::FilterType::Lanczos3,
    ))
}

impl GuiContext {
    pub(crate) fn new() -> Result<GuiContext> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        if let Ok(num) = video_subsystem.num_video_displays() {
            if num < 1 {
                return Err("No displays".into());
            }
        }
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::WEBP | InitFlag::JPG)?;
        let display_bounds = video_subsystem.display_bounds(0)?;
        let window = video_subsystem
            .window("Gallery", display_bounds.width(), display_bounds.height())
            .fullscreen()
            .position_centered()
            .build()?;
        let canvas = window.into_canvas().software().build()?;
        let texture_creator = canvas.texture_creator();

        Ok(GuiContext {
            sdl_context,
            canvas,
            texture_creator,
        })
    }

    pub(crate) fn render_pic(&mut self, path: &Path) -> Result<()> {
        let img = image::ImageReader::open(path)?.decode()?.to_rgba8();
        let (window_width, window_height) = self.canvas.window().size();
        let resized_img = resize_image(img, window_width, window_height)?;
        let (width, height) = resized_img.dimensions();
        let mut pixel_data = resized_img.into_raw();
        let surface = sdl2::surface::Surface::from_data(
            pixel_data.as_mut_slice(),
            width,
            height,
            width * 4,
            PixelFormatEnum::RGBA32,
        )?;
        let texture = self.texture_creator.create_texture_from_surface(surface)?;

        let dst = Rect::new(
            ((window_width - width) / 2) as i32,
            ((window_height - height) / 2) as i32,
            width,
            height,
        );

        self.canvas.set_draw_color(sdl2::pixels::Color::BLACK);
        self.canvas.clear();
        self.canvas.copy(&texture, None, dst)?;
        self.canvas.present();

        Ok(())
    }
}
