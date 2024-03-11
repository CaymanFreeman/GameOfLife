use crate::simulation::Simulation;

fn pass_or_fail(pass: bool) -> &'static str {
    return if pass {
        "PASSED"
    } else {
        "FAILED"
    }
}

pub(crate) fn test_printing() {

    println!("~~~TESTING PRINTING~~~");
    println!();

    let simulation_size = 15;
    let generation_iterations = 3;

    test_plane_printing(simulation_size, generation_iterations);
    println!();
    test_spheroid_printing(simulation_size, generation_iterations);
    println!();
    test_vertical_loop_printing(simulation_size, generation_iterations);
    println!();
    test_horizontal_loop_printing(simulation_size, generation_iterations);
}

pub(crate) fn test_plane_printing(simulation_size: i32, generation_iterations: u128) {
    println!("Testing Plane:");
    let mut plane = Simulation::new_plane_rand(simulation_size, simulation_size);
    plane.print_seed_generation(Some(true));
    println!();
    plane.simulate_generations(generation_iterations);
    plane.print_current_generation();
}

pub(crate) fn test_spheroid_printing(simulation_size: i32, generation_iterations: u128) {
    println!("Testing Spheroid:");
    let mut spheroid = Simulation::new_spheroid_rand(simulation_size, simulation_size);
    spheroid.print_seed_generation(Some(true));
    println!();
    spheroid.simulate_generations(generation_iterations);
    spheroid.print_current_generation();
}

pub(crate) fn test_vertical_loop_printing(simulation_size: i32, generation_iterations: u128) {
    println!("Testing Vertical Loop:");
    let mut vertical_loop = Simulation::new_vertical_loop_rand(simulation_size, simulation_size);
    vertical_loop.print_seed_generation(Some(true));
    println!();
    vertical_loop.simulate_generations(generation_iterations);
    vertical_loop.print_current_generation();
}

pub(crate) fn test_horizontal_loop_printing(simulation_size: i32, generation_iterations: u128) {
    println!("Testing Horizontal Loop:");
    let mut horizontal_loop = Simulation::new_horizontal_loop_rand(simulation_size, simulation_size);
    horizontal_loop.print_seed_generation(Some(true));
    println!();
    horizontal_loop.simulate_generations(generation_iterations);
    horizontal_loop.print_current_generation();
}

pub(crate) fn test_surfaces() {

    println!("~~~TESTING SURFACES~~~");
    println!();

    let simulation_size = 9;
    let generation_iterations = 15;

    let up_seed = "000000000000000000000111000000100100000100000000100000000010100000000000000000000";
    let down_seed = "000000000000000000000010100000100000000100000000100100000111000000000000000000000";
    let left_seed = "000000000000000000000100100001000000001000100001111000000000000000000000000000000";
    let right_seed = "000000000000000000001001000000000100001000100000111100000000000000000000000000000";

    let up_spaceship_crashed = "000000000000000000000000100000000011000000110000000000000000000000000000000000000";
    let down_spaceship_crashed = "000000000000000000000000000000000000000000110000000011000000100000000000000000000";
    let left_spaceship_crashed = "000100000000110000001010000000000000000000000000000000000000000000000000000000000";
    let right_spaceship_crashed = "000001000000011000000010100000000000000000000000000000000000000000000000000000000";

    let up_spaceship_wrapped = "000000000000000000000000000000010000000111000000101100000011100000011000000000000";
    let down_spaceship_wrapped = "000000000000011000000011100000101100000111000000010000000000000000000000000000000";
    let left_spaceship_wrapped = "000000000000000000000001100000011110000110110000011000000000000000000000000000000";
    let right_spaceship_wrapped = "000000000000000000001100000011110000011011000000110000000000000000000000000000000";

    test_plane_surface(simulation_size, generation_iterations,
                       up_seed, up_spaceship_crashed,
                       down_seed, down_spaceship_crashed,
                       left_seed, left_spaceship_crashed,
                       right_seed, right_spaceship_crashed);
    test_spheroid_surface(simulation_size, generation_iterations,
                          up_seed, up_spaceship_wrapped,
                          down_seed, down_spaceship_wrapped,
                          left_seed, left_spaceship_wrapped,
                          right_seed, right_spaceship_wrapped);
    test_vertical_loop_surface(simulation_size, generation_iterations,
                               up_seed, up_spaceship_wrapped,
                               down_seed, down_spaceship_wrapped,
                               left_seed, left_spaceship_crashed,
                               right_seed, right_spaceship_crashed);
    test_horizontal_loop_surface(simulation_size, generation_iterations,
                                 up_seed, up_spaceship_crashed,
                                 down_seed, down_spaceship_crashed,
                                 left_seed, left_spaceship_wrapped,
                                 right_seed, right_spaceship_wrapped);
}

pub(crate) fn test_plane_surface(simulation_size: i32, generation_iterations: u128,
                                 up_seed: &str, up_result: &str,
                                 down_seed: &str, down_result: &str,
                                 left_seed: &str, left_result: &str,
                                 right_seed: &str, right_result: &str) {
    println!("Testing Plane:");

    print!("Up Spaceship Crashes: ");
    let mut up_spaceship_simulation = Simulation::new_plane(simulation_size, simulation_size, up_seed.to_string());
    up_spaceship_simulation.simulate_generations(generation_iterations);
    println!("{}", pass_or_fail(up_spaceship_simulation.get_generation_string() == up_result));

    print!("Down Spaceship Crashes: ");
    let mut down_spaceship_simulation = Simulation::new_plane(simulation_size, simulation_size, down_seed.to_string());
    down_spaceship_simulation.simulate_generations(generation_iterations);
    println!("{}", pass_or_fail(down_spaceship_simulation.get_generation_string() == down_result));

    print!("Left Spaceship Crashes: ");
    let mut left_spaceship_simulation = Simulation::new_plane(simulation_size, simulation_size, left_seed.to_string());
    left_spaceship_simulation.simulate_generations(generation_iterations);
    println!("{}", pass_or_fail(left_spaceship_simulation.get_generation_string() == left_result));

    print!("Right Spaceship Crashes: ");
    let mut right_spaceship_simulation = Simulation::new_plane(simulation_size, simulation_size, right_seed.to_string());
    right_spaceship_simulation.simulate_generations(generation_iterations);
    println!("{}", pass_or_fail(right_spaceship_simulation.get_generation_string() == right_result));
    println!()
}

pub(crate) fn test_spheroid_surface(simulation_size: i32, generation_iterations: u128,
                                    up_seed: &str, up_result: &str,
                                    down_seed: &str, down_result: &str,
                                    left_seed: &str, left_result: &str,
                                    right_seed: &str, right_result: &str) {
    println!("Testing Spheroid:");

    print!("Up Spaceship Wraps: ");
    let mut up_spaceship_simulation = Simulation::new_spheroid(simulation_size, simulation_size, up_seed.to_string());
    up_spaceship_simulation.simulate_generations(generation_iterations);
    println!("{}", pass_or_fail(up_spaceship_simulation.get_generation_string() == up_result));

    print!("Down Spaceship Wraps: ");
    let mut down_spaceship_simulation = Simulation::new_spheroid(simulation_size, simulation_size, down_seed.to_string());
    down_spaceship_simulation.simulate_generations(generation_iterations);
    println!("{}", pass_or_fail(down_spaceship_simulation.get_generation_string() == down_result));

    print!("Left Spaceship Wraps: ");
    let mut left_spaceship_simulation = Simulation::new_spheroid(simulation_size, simulation_size, left_seed.to_string());
    left_spaceship_simulation.simulate_generations(generation_iterations);
    println!("{}", pass_or_fail(left_spaceship_simulation.get_generation_string() == left_result));

    print!("Right Spaceship Wraps: ");
    let mut right_spaceship_simulation = Simulation::new_spheroid(simulation_size, simulation_size, right_seed.to_string());
    right_spaceship_simulation.simulate_generations(generation_iterations);
    println!("{}", pass_or_fail(right_spaceship_simulation.get_generation_string() == right_result));
    println!()
}

pub(crate) fn test_vertical_loop_surface(simulation_size: i32, generation_iterations: u128,
                                         up_seed: &str, up_result: &str,
                                         down_seed: &str, down_result: &str,
                                         left_seed: &str, left_result: &str,
                                         right_seed: &str, right_result: &str) {
    println!("Testing Vertical Loop:");

    print!("Up Spaceship Wraps: ");
    let mut up_spaceship_simulation = Simulation::new_vertical_loop(simulation_size, simulation_size, up_seed.to_string());
    up_spaceship_simulation.simulate_generations(generation_iterations);
    println!("{}", pass_or_fail(up_spaceship_simulation.get_generation_string() == up_result));

    print!("Down Spaceship Wraps: ");
    let mut down_spaceship_simulation = Simulation::new_vertical_loop(simulation_size, simulation_size, down_seed.to_string());
    down_spaceship_simulation.simulate_generations(generation_iterations);
    println!("{}", pass_or_fail(down_spaceship_simulation.get_generation_string() == down_result));

    print!("Left Spaceship Crashes: ");
    let mut left_spaceship_simulation = Simulation::new_vertical_loop(simulation_size, simulation_size, left_seed.to_string());
    left_spaceship_simulation.simulate_generations(generation_iterations);
    println!("{}", pass_or_fail(left_spaceship_simulation.get_generation_string() == left_result));

    print!("Right Spaceship Crashes: ");
    let mut right_spaceship_simulation = Simulation::new_vertical_loop(simulation_size, simulation_size, right_seed.to_string());
    right_spaceship_simulation.simulate_generations(generation_iterations);
    println!("{}", pass_or_fail(right_spaceship_simulation.get_generation_string() == right_result));
    println!()
}

pub(crate) fn test_horizontal_loop_surface(simulation_size: i32, generation_iterations: u128,
                                           up_seed: &str, up_result: &str,
                                           down_seed: &str, down_result: &str,
                                           left_seed: &str, left_result: &str,
                                           right_seed: &str, right_result: &str) {
    println!("Testing Horizontal Loop:");

    print!("Up Spaceship Crashes: ");
    let mut up_spaceship_simulation = Simulation::new_horizontal_loop(simulation_size, simulation_size, up_seed.to_string());
    up_spaceship_simulation.simulate_generations(generation_iterations);
    println!("{}", pass_or_fail(up_spaceship_simulation.get_generation_string() == up_result));

    print!("Down Spaceship Crashes: ");
    let mut down_spaceship_simulation = Simulation::new_horizontal_loop(simulation_size, simulation_size, down_seed.to_string());
    down_spaceship_simulation.simulate_generations(generation_iterations);
    println!("{}", pass_or_fail(down_spaceship_simulation.get_generation_string() == down_result));

    print!("Left Spaceship Wraps: ");
    let mut left_spaceship_simulation = Simulation::new_horizontal_loop(simulation_size, simulation_size, left_seed.to_string());
    left_spaceship_simulation.simulate_generations(generation_iterations);
    println!("{}", pass_or_fail(left_spaceship_simulation.get_generation_string() == left_result));

    print!("Right Spaceship Wraps: ");
    let mut right_spaceship_simulation = Simulation::new_horizontal_loop(simulation_size, simulation_size, right_seed.to_string());
    right_spaceship_simulation.simulate_generations(generation_iterations);
    println!("{}", pass_or_fail(right_spaceship_simulation.get_generation_string() == right_result));
    println!()
}