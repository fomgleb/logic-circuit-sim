mod drawable_box;
mod grid;
mod logic_circuit_window;
mod rect_coords;
mod renderable;
mod renderer;
mod resolution;

use logic_circuit_window::LogicCircuitWindow;
use resolution::Resolution;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let rendering_resolution = Resolution::new(800, 600);
    let mut logic_circuit_window = LogicCircuitWindow::new()?;
    logic_circuit_window.run_main_loop(rendering_resolution)?;

    Ok(())
}
