use sdl2::event::Event;
use sdl2::pixels::Color;
use std::error::Error;

fn create_empty_window() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("window", 800, 600)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().build()?;
    let mut event_pump = sdl_context.event_pump()?;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

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

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    create_empty_window()?;

    Ok(())
}
