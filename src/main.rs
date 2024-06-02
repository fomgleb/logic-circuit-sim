mod logic_circuit_window;

use logic_circuit_window::LogicCircuitWindow;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut logic_circuit_window = LogicCircuitWindow::new()?;
    logic_circuit_window.run_main_loop()?;

    Ok(())
}
