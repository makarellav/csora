use itertools::iproduct;
use sdl2::pixels::PixelFormatEnum;
use std::error::Error;
use std::time::{Duration, Instant};

use crate::context::SDLContext;
use crate::rasterizer::Rasterizer;
use crate::vector::{Vec2, Vec3, project};

mod color;
mod context;
mod rasterizer;
mod vector;

const FPS: f32 = 60.0;
const TARGET_FRAME_DURATION: f32 = 1.0 / FPS;

const CUBE_POINTS_COUNT: usize = 9 * 9 * 9;

const MESH_VERTICES: [Vec3; 8] = [
    // 1
    Vec3 {
        x: -1.0,
        y: -1.0,
        z: -1.0,
    },
    // 2
    Vec3 {
        x: -1.0,
        y: 1.0,
        z: -1.0,
    },
    // 3
    Vec3 {
        x: 1.0,
        y: 1.0,
        z: -1.0,
    },
    // 4
    Vec3 {
        x: 1.0,
        y: -1.0,
        z: -1.0,
    },
    // 5
    Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    },
    // 6
    Vec3 {
        x: 1.0,
        y: -1.0,
        z: 1.0,
    },
    // 7
    Vec3 {
        x: -1.0,
        y: 1.0,
        z: 1.0,
    },
    // 8
    Vec3 {
        x: -1.0,
        y: -1.0,
        z: 1.0,
    },
];

#[derive(Clone, Copy)]
struct Face {
    a: usize,
    b: usize,
    c: usize,
}

const MESH_FACES: [Face; 12] = [
    // front
    Face { a: 1, b: 2, c: 3 },
    Face { a: 1, b: 3, c: 4 },
    // right
    Face { a: 4, b: 3, c: 5 },
    Face { a: 4, b: 5, c: 6 },
    // back
    Face { a: 6, b: 5, c: 7 },
    Face { a: 6, b: 7, c: 8 },
    // left
    Face { a: 8, b: 7, c: 2 },
    Face { a: 8, b: 2, c: 1 },
    // top
    Face { a: 2, b: 7, c: 5 },
    Face { a: 2, b: 5, c: 3 },
    // bottom
    Face { a: 6, b: 8, c: 1 },
    Face { a: 6, b: 1, c: 4 },
];

#[derive(Copy, Clone)]
struct Triangle {
    points: [Vec2; 3],
}

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
        let frame_start = Instant::now();

        rotations.x += 0.1;
        rotations.y += 0.1;
        rotations.z += 0.1;

        let mut triangles_to_render: [Triangle; 12] = [Triangle {
            points: [Vec2 { x: 0.0, y: 0.0 }; 3],
        }; 12];

        for (idx, face) in MESH_FACES.iter().enumerate() {
            let mut face_vertices: [Vec3; 3] = [Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }; 3];

            face_vertices[0] = MESH_VERTICES[face.a - 1];
            face_vertices[1] = MESH_VERTICES[face.b - 1];
            face_vertices[2] = MESH_VERTICES[face.c - 1];

            let mut projected_triangle = Triangle {
                points: [Vec2 { x: 0.0, y: 0.0 }; 3],
            };

            for j in 0..3 {
                let mut transformed_vertex = face_vertices[j]
                    .rotate_x(rotations.x)
                    .rotate_y(rotations.y)
                    .rotate_z(rotations.z);

                transformed_vertex.z -= 5.0;

                let projected_point =
                    project(&transformed_vertex).expect("expected a projected point");

                projected_triangle.points[j] = projected_point;
            }

            triangles_to_render[idx] = projected_triangle;
        }

        // for (idx, vec3) in cube_points.iter_mut().enumerate() {
        //     let mut rotated_point = vec3
        //         .rotate_x(rotations.x)
        //         .rotate_y(rotations.y)
        //         .rotate_z(rotations.z);

        //     rotated_point.z -= 5.0;

        //     projected_points[idx] = project(&rotated_point);
        // }

        let is_running = ctx.process_input();

        if !is_running {
            break 'running;
        }

        rasterizer.render(&mut texture, &triangles_to_render)?;

        ctx.present(&texture)?;

        let elapsed_time = frame_start.elapsed();
        let target_frame_duration = Duration::from_secs_f32(TARGET_FRAME_DURATION);

        if elapsed_time < target_frame_duration {
            std::thread::sleep(target_frame_duration - elapsed_time);
        }
    }

    Ok(())
}
