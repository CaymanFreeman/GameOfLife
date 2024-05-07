use game_of_life::simulation::{Simulation, SurfaceType};
use game_of_life::simulation_builder::SimulationBuilder;
use std::time::Duration;

fn main() {
    let mut _fittest_seed: String = String::new(); // The best or "fittest" seed
    let mut fittest_generations: u128 = 0; // The number of iterations from the fittest seed

    // This simulation will be a 15x15 square, will not wrap, and will never print or display
    let mut simulation: Simulation = SimulationBuilder::new()
        .maximum_saves(10000) // Setting maximum length of save history to 10,000 saves
        .rows(15) // 15 rows high
        .columns(15) // 15 columns wide
        .surface_type(SurfaceType::Rectangle) // Rectangle (non-wrapping) surface
        .build() // Build into a simulation
        .unwrap();

    // Loop for 1000 different seeds
    for _i in 0..1000 {
        // Simulate every generation until the simulation is finished
        simulation.simulate_continuous_generations(Duration::ZERO, true);
        // If this simulation lasted for longer than the current fittest, make it the new fittest
        if simulation.generation_iteration - 1 > fittest_generations {
            fittest_generations = simulation.generation_iteration - 1; // Set the new best iteration count (minus the initial seed)
            _fittest_seed = simulation.seed.clone(); // Set the new fittest seed
            println!(
                "The new fittest seed has lasted for {} generations:\n{}",
                fittest_generations, _fittest_seed
            );
        }
        // Reset the simulation to a random seed
        simulation.reset_to_rand()
    }
}
