use game_of_life::simulation::{Simulation, SurfaceType};
use game_of_life::simulation_builder::SimulationBuilder;
use std::time::Duration;

fn main() {
    // This simulation will be a 4x9 rectangle, will not wrap, and will never print or display
    let mut simulation: Simulation = SimulationBuilder::new() // Create a new simulation via a builder
        .rows(4) // 4 rows high
        .columns(9) // 9 columns wide
        .surface_type(SurfaceType::Rectangle) // Rectangle (non-wrapping) surface
        .seed("*----*---**-----***--*-*---**---**--") // Declaring the simulation's initial seed
        .print(false) // Declaring that the simulation should not print generations
        .display(false) // Declaring that the simulation should not display the generations in a window
        .build() // Build into a simulation
        .unwrap();

    // Simulate an iteration and print the generation
    simulation.simulate_generation();
    println!("{}", simulation);

    // Simulate 15 iterations and print the generation
    simulation.simulate_generations(15);
    println!("{}", simulation);

    // This simulation will be a 5x5 square, will wrap on all sides, and will print to console for each generation
    let mut simulation_with_print: Simulation = SimulationBuilder::new() // Create a new simulation via a builder
        .rows(5) // 5 rows high
        .columns(5) // 5 columns wide
        .surface_type(SurfaceType::Ball) // Ball (all-wrapping) surface
        .seed("-*-***--*--*-*-***-*-*-*-") // Declaring the simulation's initial seed
        .print(true) // Declaring that the simulation should print generations
        .display(false) // Declaring that the simulation should not display the generations in a window
        .build() // Build into a simulation
        .unwrap();

    // Simulate 15 iterations - simulating individually will cause a print or display frame each time
    for _i in 0..15 {
        simulation_with_print.simulate_generation()
    }

    {
        // Putting the simulation in a separate scope so that it is properly deleted before creating the next simulation
        // This simulation will be a 10x15 horizontal loop with a random seed, will only wrap left/right, and will have a window display instead of printing to console
        let mut simulation_with_display_horizontal: Simulation = SimulationBuilder::new() // Create a new simulation via a builder
            .rows(10) // 10 rows high
            .columns(15) // 15 columns wide
            .surface_type(SurfaceType::HorizontalLoop) // Horizontal loop (left/right-wrapping) surface
            // This simulation will have a random seed since we will not declare one
            .print(false) // Declaring that the simulation should not print generations
            .display(true) // Declaring that the simulation should display the generations in a window
            .window_title("Basic Simulation (50px Cell Size)") // Changing the window's title
            .cell_size(50) // Cell size of 50x50 pixels
            .build() // Build into a simulation
            .unwrap();

        // Simulate a generation every second until it is finished
        simulation_with_display_horizontal
            .simulate_continuous_generations(Duration::from_secs(1), true);
    }

    {
        // This simulation will be a 10x15 vertical loop with a random seed, will only wrap top/bottom, and will have a window display instead of printing to console
        let mut simulation_with_display_vertical: Simulation = SimulationBuilder::new() // Create a new simulation via a builder
            .rows(10) // 10 rows high
            .columns(15) // 15 columns wide
            .surface_type(SurfaceType::VerticalLoop) // Vertical loop (top/bottom-wrapping) surface
            // This simulation will have a random seed since we will not declare one
            .print(false) // Declaring that the simulation should not print generations
            .display(true) // Declaring that the simulation should display the generations in a window
            .window_title("Basic Simulation (500x350px Window Size)") // Changing the window's title
            .window_width(500) // Window width of 500 pixels
            .window_height(350) // Window height of 350 pixels
            .background_color(0, 0, 0, 255) // Black background color in RGBA
            .cell_color(0, 0, 255, 100) // Transparent blue cell color in RGBA
            .line_color(128, 128, 128, 255) // Gray line color in RGBA
            .line_thickness(3) // Line thickness of 3 pixels
            .build() // Build into a simulation
            .unwrap();

        // Simulate a generation every 250 milliseconds until it is finished
        simulation_with_display_vertical
            .simulate_continuous_generations(Duration::from_millis(250), true)
    }
}
