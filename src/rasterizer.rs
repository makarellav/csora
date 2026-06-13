use sdl2::render::Texture;

use crate::color::rgba;

pub struct Rasterizer {
    pub window_width: i32,
    pub window_height: i32,
}

impl Rasterizer {
    pub fn new(window_width: i32, window_height: i32) -> Self {
        Self {
            window_width,
            window_height,
        }
    }

    fn clear_buffer(&self, pixels: &mut [u32], pitch: usize, color: u32) {
        for y in 0..self.window_height {
            let start = y as usize * pitch;
            let end = start + self.window_width as usize;

            pixels[start..end].fill(color);
        }
    }

    fn draw_grid(&self, pixels: &mut [u32], pitch: usize, color: u32) {
        for y in 0..self.window_height {
            for x in 0..self.window_width {
                if x % 100 == 0 || y % 100 == 0 {
                    pixels[y as usize * pitch + x as usize] = color;
                }
            }
        }
    }

    fn draw_rect(
        &self,
        pixels: &mut [u32],
        pitch: usize,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: u32,
    ) {
        let x_start = x.clamp(0, self.window_width);
        let x_end = (x + width).clamp(0, self.window_width);

        let y_start = y.clamp(0, self.window_height);
        let y_end = (y + height).clamp(0, self.window_height);

        for y in y_start..y_end {
            for x in x_start..x_end {
                pixels[y as usize * pitch + x as usize] = color;
            }
        }
    }

    pub fn render(&self, texture: &mut Texture) -> Result<(), String> {
        texture.with_lock(None, |buf, pitch| {
            let pixels: &mut [u32] = bytemuck::cast_slice_mut(buf);
            let pixels_pitch = pitch / 4;

            self.clear_buffer(pixels, pixels_pitch, rgba(0, 0, 0, 255));
            self.draw_grid(pixels, pixels_pitch, rgba(0, 255, 0, 255));

            self.draw_rect(
                pixels,
                pixels_pitch,
                100,
                100,
                1000,
                500,
                rgba(0, 255, 0, 255),
            );
        })
    }
}
