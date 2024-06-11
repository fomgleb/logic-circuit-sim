use crate::renderable::Renderable;
use sdl2::{
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};
use std::error::Error;

/// A structure representing a drawable box.
pub struct DrawableBox {
    rect: Rect,
}

impl DrawableBox {
    /// Creates a new `DrawableBox`.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the top-left corner of the box.
    /// * `y` - The y-coordinate of the top-left corner of the box.
    /// * `width` - The width of the box.
    /// * `height` - The height of the box.
    ///
    /// # Returns
    ///
    /// A new instance of `DrawableBox`.
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
        }
    }
}

impl Renderable for DrawableBox {
    /// Renders the box on the given canvas, adjusted by the field offset.
    ///
    /// # Arguments
    ///
    /// * `canvas` - The canvas to render the box on.
    /// * `field_offset` - The offset to adjust the box's position by.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    fn render(
        &self,
        canvas: &mut Canvas<Window>,
        field_offset: Point,
    ) -> Result<(), Box<dyn Error>> {
        let (window_width, window_height) = canvas.window().size();
        let mut drawing_rect = self.rect;
        drawing_rect.x -= field_offset.x;
        drawing_rect.y -= field_offset.y;

        // Check if the rectangle is within the bounds of the window.
        if drawing_rect.x + drawing_rect.w as i32 >= 0
            && drawing_rect.y + drawing_rect.h as i32 >= 0
            && drawing_rect.x <= window_width as i32
            && drawing_rect.y <= window_height as i32
        {
            canvas.fill_rect(drawing_rect)?;
        }
        Ok(())
    }
}
