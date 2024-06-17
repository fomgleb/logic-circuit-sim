use crate::{renderable::Renderable, resolution::Resolution};
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

    pub fn render_everything_with_resolution(
        &self,
        canvas: &mut Canvas<Window>,
        field_offset: Point,
        resolution: Resolution,
    ) -> Result<(), Box<dyn Error>> {
        for r in self.renderables.iter() {
            r.render_with_resolution(canvas, field_offset, resolution)?;
        }

        Ok(())
    }
}
