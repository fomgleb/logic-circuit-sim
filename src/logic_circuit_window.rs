use sdl2::{
    event::Event, mouse::MouseButton, pixels::Color, rect::Point,
    render::Canvas, video::Window, EventPump, VideoSubsystem,
};
use std::error::Error;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const GRID_DISTANCE_HORIZONTAL: usize = 50;
const GRID_DISTANCE_VERTICAL: usize = 50;

pub struct LogicCircuitWindow {
    sdl_context: sdl2::Sdl,
    video_subsystem: VideoSubsystem,
    canvas: Canvas<Window>,
    event_pump: EventPump,

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
            field_offset_is_changing: false,
        })
    }

    fn render_grid(&mut self) -> Result<(), Box<dyn Error>> {
        self.canvas.set_draw_color(Color::BLACK);

        let first_horizontal_line_y = (self.field_offset.y as f32
            / GRID_DISTANCE_HORIZONTAL as f32)
            .ceil() as i32
            * GRID_DISTANCE_HORIZONTAL as i32
            - self.field_offset.y;
        let first_vertical_line_x = (self.field_offset.x as f32
            / GRID_DISTANCE_VERTICAL as f32)
            .ceil() as i32
            * GRID_DISTANCE_VERTICAL as i32
            - self.field_offset.x;

        for y in (first_horizontal_line_y
            ..(first_horizontal_line_y + WINDOW_HEIGHT as i32))
            .step_by(GRID_DISTANCE_HORIZONTAL)
        {
            self.canvas.draw_line(
                Point::new(0, y),
                Point::new(WINDOW_WIDTH as i32, y),
            )?;
        }
        for x in (first_vertical_line_x
            ..(first_vertical_line_x + WINDOW_WIDTH as i32))
            .step_by(GRID_DISTANCE_VERTICAL)
        {
            self.canvas.draw_line(
                Point::new(x, 0),
                Point::new(x, WINDOW_HEIGHT as i32),
            )?;
        }

        Ok(())
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
                        x,
                        y,
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
                self.render_grid()?;
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
