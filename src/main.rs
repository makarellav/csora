use sdl2::{self, pixels::PixelFormatEnum, video::FullscreenType};

fn clear_buffer(buf: &mut [u8]) {
    let pixels: &mut [u32] = bytemuck::cast_slice_mut(buf);

    pixels.iter_mut().for_each(|pixel| {
        *pixel = 0x000000FF;
    });
}

fn draw_grid(buf: &mut [u8], window_width: u32, window_height: u32, color: u32) {
    let pixels: &mut [u32] = bytemuck::cast_slice_mut(buf);

    for y in 0..window_height {
        for x in 0..window_width {
            if y % 100 == 0 || x % 100 == 0 {
                pixels[((window_width * y) + x) as usize] = color;
            }
        }
    }
}

fn draw_rect(
    buf: &mut [u8],
    window_width: i32,
    window_height: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: u32,
) {
    let pixels: &mut [u32] = bytemuck::cast_slice_mut(buf);

    let x_start = x.clamp(0, window_width);
    let x_end = (x + width).clamp(0, window_width);

    let y_start = y.clamp(0, window_height);
    let y_end = (y + height).clamp(0, window_height);

    for y in y_start..y_end {
        for x in x_start..x_end {
            pixels[((y * window_width) + x) as usize] = color;
        }
    }
}

fn rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    (r as u32) << 24 | (g as u32) << 16 | (b as u32) << 8 | (a as u32)
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let display_mode = video_subsystem.desktop_display_mode(0).unwrap();

    let window_width = display_mode.w as u32;
    let window_height = display_mode.h as u32;

    let window = video_subsystem
        .window("Software Rasterizer", window_width, window_height)
        .position_centered()
        .borderless()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().software().build().unwrap();

    canvas
        .window_mut()
        .set_fullscreen(FullscreenType::True)
        .unwrap();

    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBA8888, window_width, window_height)
        .unwrap();

    texture
        .with_lock(None, |buf, _| {
            clear_buffer(buf);
            draw_grid(buf, window_width, window_height, rgba(0, 255, 0, 255));
            draw_rect(
                buf,
                window_width as i32,
                window_height as i32,
                100,
                100,
                1000,
                500,
                rgba(0, 255, 0, 255),
            );
        })
        .unwrap();

    canvas.copy(&texture, None, None).unwrap();
    canvas.present();

    std::thread::sleep(std::time::Duration::from_secs(5));
}
