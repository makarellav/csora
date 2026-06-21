use itertools::iproduct;
use sdl2::pixels::PixelFormatEnum;
use std::error::Error;

use crate::context::SDLContext;
use crate::rasterizer::Rasterizer;
use crate::vector::{Vec2, Vec3, project};

mod color;
mod context;
mod rasterizer;
mod vector;

const CUBE_POINTS_COUNT: usize = 9 * 9 * 9;

fn main() -> Result<(), Box<dyn Error>> {
    let mut ctx = SDLContext::new()?;

    let texture_creator = ctx.canvas.texture_creator();

    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGBA8888,
        ctx.window_width as u32,
        ctx.window_height as u32,
    )?;

    let rasterizer = Rasterizer::new(ctx.window_width, ctx.window_height);

    // TODO: this is a mess
    let mut cube_points = [Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }; CUBE_POINTS_COUNT];

    let mut projected_points: [Option<Vec2>; CUBE_POINTS_COUNT] = [None; CUBE_POINTS_COUNT];

    let steps = (-4..=4).map(|x| x as f32 * 0.25);

    for (idx, (x, y, z)) in iproduct!(steps.clone(), steps.clone(), steps).enumerate() {
        cube_points[idx] = Vec3 { x, y, z }
    }

    let mut rotations = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    'running: loop {
        rotations.x += 0.1;
        rotations.y += 0.1;
        rotations.z += 0.1;

        for (idx, vec3) in cube_points.iter_mut().enumerate() {
            let mut rotated_point = vec3
                .rotate_x(rotations.x)
                .rotate_y(rotations.y)
                .rotate_z(rotations.z);

            rotated_point.z -= 5.0;

            projected_points[idx] = project(&rotated_point);
        }

        let is_running = ctx.process_input();

        if !is_running {
            break 'running;
        }

        rasterizer.render(&mut texture, &projected_points)?;

        ctx.present(&texture)?;
    }

    Ok(())
}
