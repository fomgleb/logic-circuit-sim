use crate::{
    rect_coords::RectCoords, renderable::Renderable, resolution::Resolution,
};
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

        self.render_with_resolution(
            canvas,
            field_offset,
            Resolution::new(window_width as u16, window_height as u16),
        )
    }

    fn render_with_resolution(
        &self,
        canvas: &mut Canvas<Window>,
        field_offset: Point,
        resolution: Resolution,
    ) -> Result<(), Box<dyn Error>> {
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

        let drawing_rect = Rect::new(
            self.rect.x - field_offset.x,
            self.rect.y - field_offset.y,
            self.rect.width(),
            self.rect.height(),
        );

        // Check if the rectangle is within the bounds of the window.
        if drawing_rect.x + drawing_rect.w >= rendering_area.left_x as i32
            && drawing_rect.y + drawing_rect.h >= rendering_area.top_y as i32
            && drawing_rect.x <= rendering_area.right_x as i32
            && drawing_rect.y <= rendering_area.bottom_y as i32
        {
            canvas.fill_rect(drawing_rect)?;
        }

        Ok(())
    }
}
