use crate::resolution::Resolution;
use sdl2::{rect::Point, render::Canvas, video::Window};
use std::error::Error;

pub trait Renderable {
    fn render(
        &self,
        canvas: &mut Canvas<Window>,
        field_offset: Point,
    ) -> Result<(), Box<dyn Error>>;

    fn render_with_resolution(
        &self,
        canvas: &mut Canvas<Window>,
        field_offset: Point,
        resolution: Resolution,
    ) -> Result<(), Box<dyn Error>>;
}
