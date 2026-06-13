use sdl2::pixels::PixelFormatEnum;
use std::error::Error;

use crate::context::SDLContext;
use crate::rasterizer::Rasterizer;

mod color;
mod context;
mod rasterizer;

fn main() -> Result<(), Box<dyn Error>> {
    let mut ctx = SDLContext::new()?;

    let texture_creator = ctx.canvas.texture_creator();

    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGBA8888,
        ctx.window_width as u32,
        ctx.window_height as u32,
    )?;

    let rasterizer = Rasterizer::new(ctx.window_width, ctx.window_height);

    'running: loop {
        let is_running = ctx.process_input();

        if !is_running {
            break 'running;
        }

        rasterizer.render(&mut texture)?;

        ctx.present(&texture)?;
    }

    Ok(())
}
