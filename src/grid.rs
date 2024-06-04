use crate::renderable::Renderable;
use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};
use std::error::Error;

const GRID_DISTANCE_HORIZONTAL: usize = 50;
const GRID_DISTANCE_VERTICAL: usize = 50;

pub struct Grid;

impl Renderable for Grid {
    fn render(
        &self,
        canvas: &mut Canvas<Window>,
        field_offset: Point,
    ) -> Result<(), Box<(dyn Error)>> {
        canvas.set_draw_color(Color::BLACK);

        let (window_width, window_height) = canvas.window().size();

        let first_horizontal_line_y = (field_offset.y as f32
            / GRID_DISTANCE_HORIZONTAL as f32)
            .ceil() as i32
            * GRID_DISTANCE_HORIZONTAL as i32
            - field_offset.y;
        let first_vertical_line_x = (field_offset.x as f32
            / GRID_DISTANCE_VERTICAL as f32)
            .ceil() as i32
            * GRID_DISTANCE_VERTICAL as i32
            - field_offset.x;

        for y in (first_horizontal_line_y
            ..(first_horizontal_line_y + window_height as i32))
            .step_by(GRID_DISTANCE_HORIZONTAL)
        {
            canvas.draw_line(
                Point::new(0, y),
                Point::new(window_width as i32, y),
            )?;
        }
        for x in (first_vertical_line_x
            ..(first_vertical_line_x + window_width as i32))
            .step_by(GRID_DISTANCE_VERTICAL)
        {
            canvas.draw_line(
                Point::new(x, 0),
                Point::new(x, window_height as i32),
            )?;
        }

        Ok(())
    }
}
