use crate::{
    rect_coords::RectCoords, renderable::Renderable, resolution::Resolution,
};
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
        let (window_width, window_height) = canvas.output_size()?;
        self.render_with_resolution(
            canvas,
            field_offset,
            Resolution::new(window_width as u16, window_height as u16),
        )?;

        Ok(())
    }

    fn render_with_resolution(
        &self,
        canvas: &mut Canvas<Window>,
        field_offset: Point,
        resolution: Resolution,
    ) -> Result<(), Box<dyn Error>> {
        canvas.set_draw_color(Color::BLACK);

        let rendering_area = {
            let (window_width, window_height) = canvas.output_size()?;
            let x = (window_width as u16 - resolution.width) / 2;
            let y = (window_height as u16 - resolution.height) / 2;

            RectCoords {
                left_x: x,
                top_y: y,
                right_x: resolution.width + x,
                bottom_y: resolution.height + y,
            }
        };

        let first_horizontal_line_y = (field_offset.y as f32
            / GRID_DISTANCE_HORIZONTAL as f32)
            .ceil() as i32
            * GRID_DISTANCE_HORIZONTAL as i32
            - field_offset.y
            + rendering_area.top_y as i32;
        let first_vertical_line_x = (field_offset.x as f32
            / GRID_DISTANCE_VERTICAL as f32)
            .ceil() as i32
            * GRID_DISTANCE_VERTICAL as i32
            - field_offset.x
            + rendering_area.left_x as i32;

        for y in (first_horizontal_line_y..(rendering_area.bottom_y as i32))
            .step_by(GRID_DISTANCE_HORIZONTAL)
        {
            canvas.draw_line(
                Point::new(rendering_area.left_x as i32, y),
                Point::new(rendering_area.right_x as i32, y),
            )?;
        }
        for x in (first_vertical_line_x..(rendering_area.right_x as i32))
            .step_by(GRID_DISTANCE_VERTICAL)
        {
            canvas.draw_line(
                Point::new(x, rendering_area.top_y as i32),
                Point::new(x, rendering_area.bottom_y as i32),
            )?;
        }

        Ok(())
    }
}
