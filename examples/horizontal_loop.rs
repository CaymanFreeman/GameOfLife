use simple_game_of_life::simulation::{Simulation, SurfaceType};
use simple_game_of_life::simulation_builder::SimulationBuilder;
use std::time::Duration;

// For this example, we simply want to view a random
// simulation on a horizontal loop until it is finished.

fn main() {
    // This simulation will be a 15x15 horizontal loop with a random seed, will only wrap left/right, and will have a window display
    let mut simulation: Simulation = SimulationBuilder::new() // Create a new simulation via a builder
        .rows(15) // 15 rows high
        .columns(15) // 15 columns wide
        .surface_type(SurfaceType::HorizontalLoop) // Horizontal loop (left/right-wrapping) surface
        // This simulation will have a random seed since we will not declare one
        .display(true) // Declaring that the simulation should display the generations in a window
        .cell_size(50) // Cell size of 50 pixels
        .build() // Build into a simulation
        .unwrap();

    // Simulate a generation every 250 milliseconds until it is finished
    simulation.simulate_continuous_generations(Duration::from_millis(250), true);

    // Quit and close the window
    simulation.quit_window();
}