use sdl2::{self, pixels::Color, video::FullscreenType};

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

    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
    canvas.clear();
    canvas.present();

    std::thread::sleep(std::time::Duration::from_secs(5));
}
