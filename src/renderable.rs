use std::error::Error;

use sdl2::{rect::Point, render::Canvas, video::Window};

pub trait Renderable {
    fn render(
        &self,
        canvas: &mut Canvas<Window>,
        field_offset: Point,
    ) -> Result<(), Box<dyn Error>>;
}
