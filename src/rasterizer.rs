use sdl2::render::Texture;

use crate::{Triangle, color::rgba, vector::Vec2};

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
                    self.draw_pixel(pixels, pitch, x as usize, y as usize, color);
                }
            }
        }
    }

    pub fn draw_pixel(&self, pixels: &mut [u32], pitch: usize, x: usize, y: usize, color: u32) {
        pixels[y * pitch + x] = color;
    }

    pub fn draw_rect(
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
                self.draw_pixel(pixels, pitch, x as usize, y as usize, color);
            }
        }
    }

    fn draw_line(
        &self,
        pixels: &mut [u32],
        pitch: usize,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        color: u32,
    ) {
        let x_delta = x1 - x0;
        let y_delta = y1 - y0;

        let side_length = std::cmp::max(x_delta.abs(), y_delta.abs());

        let x_inc = x_delta as f32 / side_length as f32;
        let y_inc = y_delta as f32 / side_length as f32;

        let mut current_x = x0 as f32;
        let mut current_y = y0 as f32;

        for _ in 0..=side_length {
            self.draw_pixel(
                pixels,
                pitch,
                current_x.round() as usize,
                current_y.round() as usize,
                color,
            );

            current_x += x_inc;
            current_y += y_inc;
        }
    }

    pub fn render(
        &self,
        texture: &mut Texture,
        triangles_to_render: &[Triangle],
    ) -> Result<(), String> {
        texture.with_lock(None, |buf, pitch| {
            let pixels: &mut [u32] = bytemuck::cast_slice_mut(buf);
            let pixels_pitch = pitch / 4;

            self.clear_buffer(pixels, pixels_pitch, rgba(0, 0, 0, 255));
            // self.draw_grid(pixels, pixels_pitch, rgba(0, 255, 0, 255));

            triangles_to_render.iter().for_each(|triangle| {
                triangle.points.iter().for_each(|p| {
                    self.draw_rect(
                        pixels,
                        pixels_pitch,
                        (p.x + (self.window_width / 2) as f32) as i32,
                        (p.y + (self.window_height / 2) as f32) as i32,
                        4,
                        4,
                        rgba(255, 0, 255, 255),
                    );
                });

                self.draw_line(
                    pixels,
                    pixels_pitch,
                    (triangle.points[0].x + (self.window_width / 2) as f32) as i32,
                    (triangle.points[0].y + (self.window_height / 2) as f32) as i32,
                    (triangle.points[1].x + (self.window_width / 2) as f32) as i32,
                    (triangle.points[1].y + (self.window_height / 2) as f32) as i32,
                    rgba(255, 0, 255, 255),
                );

                self.draw_line(
                    pixels,
                    pixels_pitch,
                    (triangle.points[1].x + (self.window_width / 2) as f32) as i32,
                    (triangle.points[1].y + (self.window_height / 2) as f32) as i32,
                    (triangle.points[2].x + (self.window_width / 2) as f32) as i32,
                    (triangle.points[2].y + (self.window_height / 2) as f32) as i32,
                    rgba(255, 0, 255, 255),
                );

                self.draw_line(
                    pixels,
                    pixels_pitch,
                    (triangle.points[2].x + (self.window_width / 2) as f32) as i32,
                    (triangle.points[2].y + (self.window_height / 2) as f32) as i32,
                    (triangle.points[0].x + (self.window_width / 2) as f32) as i32,
                    (triangle.points[0].y + (self.window_height / 2) as f32) as i32,
                    rgba(255, 0, 255, 255),
                );
            });
        })
    }
}
