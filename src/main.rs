use sdl2::{self, pixels::PixelFormatEnum, video::FullscreenType};

fn clear_buffer(buf: &mut [u8], _: usize) -> () {
    let pixels: &mut [u32] = bytemuck::cast_slice_mut(buf);

    pixels.iter_mut().for_each(|pixel| {
        *pixel = 0x000000FF;
    });
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

    texture.with_lock(None, clear_buffer).unwrap();

    canvas.copy(&texture, None, None).unwrap();
    canvas.present();

    std::thread::sleep(std::time::Duration::from_secs(5));
}
