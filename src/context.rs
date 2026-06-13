use std::error::Error;

use sdl2::{
    EventPump, event, init,
    keyboard::Keycode,
    render::{Canvas, Texture},
    video::{FullscreenType, Window},
};

pub struct SDLContext {
    pub canvas: Canvas<Window>,
    pub window_width: i32,
    pub window_height: i32,
    event_pump: EventPump,
}

impl SDLContext {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let sdl_context = init()?;
        let video_subsystem = sdl_context.video()?;

        let display_mode = video_subsystem.desktop_display_mode(0)?;

        let window_width = display_mode.w as u32;
        let window_height = display_mode.h as u32;

        let window = video_subsystem
            .window("Software Rasterizer", window_width, window_height)
            .position_centered()
            .borderless()
            .build()?;

        let mut canvas = window.into_canvas().software().build()?;

        canvas.window_mut().set_fullscreen(FullscreenType::True)?;

        let event_pump = sdl_context.event_pump()?;

        Ok(Self {
            canvas,
            event_pump,
            window_width: display_mode.w,
            window_height: display_mode.h,
        })
    }

    pub fn process_input(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                event::Event::Quit { .. }
                | event::Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return false;
                }
                _ => {}
            }
        }

        true
    }

    pub fn present(&mut self, texture: &Texture) -> Result<(), String> {
        self.canvas.copy(&texture, None, None)?;
        self.canvas.present();

        Ok(())
    }
}
