use simple_game_of_life::simulation::{Simulation, SurfaceType};
use simple_game_of_life::simulation_builder::SimulationBuilder;
use std::thread::sleep;
use std::time::Duration;

// For this example, we simply want to showcase
// some of the options for customizing
// the display window for a simulation.

fn main() {
    // This simulation will be a 7x7 square with a random seed, will not wrap, and will have a window display
    let mut simulation_red_and_black: Simulation = SimulationBuilder::new() // Create a new simulation via a builder
        .rows(7) // 7 rows high
        .columns(7) // 7 columns wide
        .surface_type(SurfaceType::Rectangle) // Rectangle (non-wrapping) surface
        // This simulation will have a random seed since we will not declare one
        .display(true) // Declaring that the simulation should display the generations in a window
        .cell_size(50) // Cell size of 50 pixels
        .cell_color(255, 0, 0, 255) // Red cells
        .background_color(0, 0, 0, 255) // Black background
        .build() // Build into a simulation
        .unwrap();

    // Simulate 20 individual iterations with a 250 millisecond cooldown
    for _i in 0..20 {
        simulation_red_and_black.simulate_generation();
        sleep(Duration::from_millis(250))
    }

    // Quit and close the window
    simulation_red_and_black.quit_window();

    // Wait 1 second between simulations
    sleep(Duration::from_secs(1));

    // This simulation will be a 7x7 square with a random seed, will not wrap, and will have a window display
    let mut simulation_green_and_blue: Simulation = SimulationBuilder::new() // Create a new simulation via a builder
        .rows(7) // 7 rows high
        .columns(7) // 7 columns wide
        .surface_type(SurfaceType::Rectangle) // Rectangle (non-wrapping) surface
        // This simulation will have a random seed since we will not declare one
        .display(true) // Declaring that the simulation should display the generations in a window
        .cell_size(50) // Cell size of 50 pixels
        .cell_color(0, 255, 20, 255) // Green cells
        .line_color(0, 20, 200, 255) // Blue lines
        .build() // Build into a simulation
        .unwrap();

    // Simulate 20 individual iterations with a 250 millisecond cooldown
    for _i in 0..20 {
        simulation_green_and_blue.simulate_generation();
        sleep(Duration::from_millis(250))
    }

    // Quit and close the window
    simulation_green_and_blue.quit_window();

    // Wait 1 second between simulations
    sleep(Duration::from_secs(1));

    // This simulation will be a 7x7 square with a random seed, will not wrap, and will have a window display
    let mut simulation_stretched: Simulation = SimulationBuilder::new() // Create a new simulation via a builder
        .rows(7) // 7 rows high
        .columns(7) // 7 columns wide
        .surface_type(SurfaceType::Rectangle) // Rectangle (non-wrapping) surface
        // This simulation will have a random seed since we will not declare one
        .display(true) // Declaring that the simulation should display the generations in a window
        .cell_width(50) // Cell width of 50 pixels
        .cell_height(85) // Cell height of 85 pixels
        .line_thickness(3) // Line thickness of 3 pixels
        .build() // Build into a simulation
        .unwrap();

    // Simulate 20 individual iterations with a 250 millisecond cooldown
    for _i in 0..20 {
        simulation_stretched.simulate_generation();
        sleep(Duration::from_millis(250))
    }

    // Quit and close the window
    simulation_stretched.quit_window();
}
