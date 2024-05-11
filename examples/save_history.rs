use simple_game_of_life::simulation::Simulation;
use simple_game_of_life::simulation_builder::SimulationBuilder;

// For this example, we want to simulate two simulations
// to the same point from the same seed but through
// staggering rollbacks and simulations with A
// and directly simulating with B.

fn main() {
    // This simulation will be a 5x5 square
    let mut simulation_a: Simulation = SimulationBuilder::new() // Create a new simulation via a builder
        .height(5) // 5 rows high
        .width(5) // 5 columns wide
        .surface_rectangle() // Rectangle (non-wrapping) surface
        .seed("-*-***--*--*-*-***-*-*-*-") // Declaring the simulation's initial seed
        .build() // Build into a simulation
        .unwrap();

    // Clone simulation A to simulation B
    let mut simulation_b = simulation_a.clone();

    // We will simulate and roll back generations to equal 15 simulations for simulation A
    // 15 = 3 - 2 + 10 - 4 + 8
    simulation_a.simulate_generations(3); // +3
    simulation_a.rollback_generations(2); // -2
    simulation_a.simulate_generations(10); // +10
    simulation_a.rollback_generations(4); // -4
    simulation_a.simulate_generations(8); // +8

    // Simulate 15 iterations for B
    simulation_b.simulate_generations(15);

    // Check that both simulations end up with the same generation
    assert_eq!(
        simulation_a.generation_string(),
        simulation_b.generation_string()
    );
}
