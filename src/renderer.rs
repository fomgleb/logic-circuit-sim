use crate::renderable::Renderable;
use sdl2::{rect::Point, render::Canvas, video::Window};
use std::{collections::LinkedList, error::Error};

pub struct Renderer {
    pub renderables: LinkedList<Box<dyn Renderable>>,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            renderables: LinkedList::new(),
        }
    }

    pub fn render_everything(
        &self,
        canvas: &mut Canvas<Window>,
        field_offset: Point,
    ) -> Result<(), Box<dyn Error>> {
        for r in self.renderables.iter() {
            r.render(canvas, field_offset)?;
        }

        Ok(())
    }
}
