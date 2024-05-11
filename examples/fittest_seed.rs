use simple_game_of_life::simulation::Simulation;
use simple_game_of_life::simulation_builder::SimulationBuilder;
use std::time::Duration;

// For this example, we want to find out which seed will produce the
// longest-lasting simulation (the most generations without encountering any periodic state)
// within a rectangle. We will call this seed the "fittest" seed. We also might be
// interested in the relationship between "fitness" and the initial alive proportion.

fn main() {
    let mut fittest_seed: String; // The best or "fittest" seed
    let mut fittest_generations: u128 = 0; // The number of iterations from the fittest seed
    let mut alive_count: u64; // The number of alive cells in the fittest seed
    let mut alive_proportion: f64; // The alive count divided by the seed's area

    // This simulation will be a 15x15 square, will not wrap, and will never print or display
    let mut simulation: Simulation = SimulationBuilder::new()
        // Setting maximum length of save history to 10,000 generations
        // This will increase the length period
        // the .is_finished() function can check for.
        .maximum_saves(10000)
        .height(15) // 15 rows high
        .width(15) // 15 columns wide
        .surface_rectangle() // Rectangle (non-wrapping) surface
        .build() // Build into a simulation
        .unwrap();

    // Loop for 1000 different seeds
    for _i in 0..1000 {
        alive_count = simulation.alive_count(); // Set the alive count
        alive_proportion = simulation.alive_proportion(); // Set the alive proportion
                                                          // Simulate every generation until the simulation is finished
        simulation.simulate_continuous_generations(Duration::ZERO, true);
        // If this simulation lasted for longer than the current fittest, make it the new fittest
        if simulation.iteration() - 1 > fittest_generations {
            fittest_generations = simulation.iteration() - 1; // Set the new best iteration count (minus the initial seed)
            fittest_seed = simulation.seed(); // Set the new fittest seed
            println!(
                "The new fittest seed has lasted for {} generations with an alive proportion of {} ({}/{}):\n{}",
                fittest_generations, alive_proportion, alive_count, simulation.area(), fittest_seed
            );
        }
        // Reset the simulation to a random seed
        simulation.reset_to_rand()
    }
}
