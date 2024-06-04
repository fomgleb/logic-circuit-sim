use crate::{grid::Grid, renderer::Renderer};
use sdl2::{
    event::Event, mouse::MouseButton, pixels::Color, rect::Point,
    render::Canvas, video::Window, EventPump, VideoSubsystem,
};
use std::error::Error;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

pub struct LogicCircuitWindow {
    sdl_context: sdl2::Sdl,
    video_subsystem: VideoSubsystem,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    renderer: Renderer,

    last_lmb_down_point: Point,
    field_offset: Point,
    field_offset_is_changing: bool,
}

impl LogicCircuitWindow {
    pub fn new() -> Result<LogicCircuitWindow, Box<dyn Error>> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("window", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()?;

        let mut canvas = window.into_canvas().build()?;
        let event_pump = sdl_context.event_pump()?;

        let mut renderer = Renderer::new();
        renderer.renderables.push_back(Box::new(Grid));

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();

        Ok(LogicCircuitWindow {
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
            renderer,
            last_lmb_down_point: Point::new(0, 0),
            field_offset: Point::new(0, 0),
            field_offset_is_changing: false,
        })
    }

    pub fn run_main_loop(&mut self) -> Result<(), Box<dyn Error>> {
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
                    } => {
                        self.last_lmb_down_point =
                            Point::new(x, y) + self.field_offset;
                        self.field_offset_is_changing = true;
                    }
                    Event::MouseButtonUp {
                        mouse_btn: MouseButton::Left,
                        ..
                    } => {
                        self.field_offset_is_changing = false;
                    }
                    Event::MouseMotion { x, y, .. } => {
                        if self.field_offset_is_changing {
                            self.field_offset =
                                self.last_lmb_down_point.offset(-x, -y);
                        }
                    }
                    _ => {}
                }
            }

            if self.field_offset_is_changing {
                self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                self.canvas.clear();
                self.renderer
                    .render_everything(&mut self.canvas, self.field_offset)?;
                self.canvas.present();
            }

            println!(
                "x: {}, y: {}",
                self.field_offset.x(),
                self.field_offset.y()
            );

            std::thread::sleep(std::time::Duration::from_secs_f32(1.0 / 60.0));
        }

        Ok(())
    }
}
