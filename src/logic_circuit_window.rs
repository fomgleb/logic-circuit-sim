use sdl2::{
    event::Event, mouse::MouseButton, pixels::Color, rect::Point,
    render::Canvas, video::Window, EventPump, VideoSubsystem,
};
use std::error::Error;

pub struct LogicCircuitWindow {
    sdl_context: sdl2::Sdl,
    video_subsystem: VideoSubsystem,
    canvas: Canvas<Window>,
    event_pump: EventPump,

    last_lmb_down_point: Point,
    field_offset: Point,
}

impl LogicCircuitWindow {
    pub fn new() -> Result<LogicCircuitWindow, Box<dyn Error>> {
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

        let last_lmb_down_point = Point::new(0, 0);
        let field_offset = Point::new(0, 0);

        Ok(LogicCircuitWindow {
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
            last_lmb_down_point,
            field_offset,
        })
    }

    pub fn run_main_loop(&mut self) {
        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(sdl2::keyboard::Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::MouseButtonDown {
                        mouse_btn: MouseButton::Left,
                        x,
                        y,
                        ..
                    } => self.last_lmb_down_point = Point::new(x, y),
                    Event::MouseButtonUp {
                        mouse_btn: MouseButton::Left,
                        x,
                        y,
                        ..
                    } => {
                        self.field_offset +=
                            self.last_lmb_down_point.offset(-x, -y);
                    }
                    _ => {}
                }
            }
            std::thread::sleep(std::time::Duration::from_secs_f32(1.0 / 60.0));
        }
    }
}
