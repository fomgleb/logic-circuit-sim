use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, VideoSubsystem};
use std::error::Error;

struct SDLContext {
    sdl_context: sdl2::Sdl,
    video_subsystem: VideoSubsystem,
    canvas: Canvas<Window>,
    event_pump: EventPump,
}

fn create_empty_window() -> Result<SDLContext, Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("window", 800, 600)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().build()?;
    let event_pump = sdl_context.event_pump()?;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    Ok(SDLContext {
        sdl_context,
        video_subsystem,
        canvas,
        event_pump,
    })
}

fn run_main_loop(event_pump: &mut EventPump) {
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        std::thread::sleep(std::time::Duration::from_secs_f32(1.0 / 60.0));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut sdl_context = create_empty_window()?;
    run_main_loop(&mut sdl_context.event_pump);

    // sdl_context is dropped here, which calls SDL_Quit()

    Ok(())
}
