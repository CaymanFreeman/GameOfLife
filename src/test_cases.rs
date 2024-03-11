use crate::simulation::Simulation;

fn pass_or_fail(pass: bool) -> &'static str {
    return if pass {
        "PASSED"
    } else {
        "FAILED"
    }
}

const NUMBER_OF_SPACESHIP_GENERATIONS: u128 = 15;
const SPACESHIP_SIMULATION_SIZE: i32 = 9;

const UP_SPACESHIP_SEED: &str = "000000000000000000000111000000100100000100000000100000000010100000000000000000000";
const DOWN_SPACESHIP_SEED: &str = "000000000000000000000010100000100000000100000000100100000111000000000000000000000";
const LEFT_SPACESHIP_SEED: &str = "000000000000000000000100100001000000001000100001111000000000000000000000000000000";
const RIGHT_SPACESHIP_SEED: &str = "000000000000000000001001000000000100001000100000111100000000000000000000000000000";

const UP_SPACESHIP_CRASHED: &str = "000000000000000000000000100000000011000000110000000000000000000000000000000000000";
const DOWN_SPACESHIP_CRASHED: &str = "000000000000000000000000000000000000000000110000000011000000100000000000000000000";
const LEFT_SPACESHIP_CRASHED: &str = "000100000000110000001010000000000000000000000000000000000000000000000000000000000";
const RIGHT_SPACESHIP_CRASHED: &str = "000001000000011000000010100000000000000000000000000000000000000000000000000000000";

const UP_SPACESHIP_WRAPPED: &str = "000000000000000000000000000000010000000111000000101100000011100000011000000000000";
const DOWN_SPACESHIP_WRAPPED: &str = "000000000000011000000011100000101100000111000000010000000000000000000000000000000";
const LEFT_SPACESHIP_WRAPPED: &str = "000000000000000000000001100000011110000110110000011000000000000000000000000000000";
const RIGHT_SPACESHIP_WRAPPED: &str = "000000000000000000001100000011110000011011000000110000000000000000000000000000000";

pub(crate) fn test_finite() {
    test_finite_plane();
    test_finite_spheroid();
    test_finite_vertical_loop();
    test_finite_horizontal_loop();
}

pub(crate) fn test_finite_plane() {
    println!("Testing Finite Plane:");

    print!("Up Spaceship Crashes: ");
    let mut up_spaceship_simulation = Simulation::new_finite_plane(SPACESHIP_SIMULATION_SIZE, SPACESHIP_SIMULATION_SIZE, UP_SPACESHIP_SEED.to_string());
    up_spaceship_simulation.simulate_generations(NUMBER_OF_SPACESHIP_GENERATIONS);
    println!("{}", pass_or_fail(up_spaceship_simulation.get_generation() == UP_SPACESHIP_CRASHED));

    print!("Down Spaceship Crashes: ");
    let mut down_spaceship_simulation = Simulation::new_finite_plane(SPACESHIP_SIMULATION_SIZE, SPACESHIP_SIMULATION_SIZE, DOWN_SPACESHIP_SEED.to_string());
    down_spaceship_simulation.simulate_generations(NUMBER_OF_SPACESHIP_GENERATIONS);
    println!("{}", pass_or_fail(down_spaceship_simulation.get_generation() == DOWN_SPACESHIP_CRASHED));

    print!("Left Spaceship Crashes: ");
    let mut left_spaceship_simulation = Simulation::new_finite_plane(SPACESHIP_SIMULATION_SIZE, SPACESHIP_SIMULATION_SIZE, LEFT_SPACESHIP_SEED.to_string());
    left_spaceship_simulation.simulate_generations(NUMBER_OF_SPACESHIP_GENERATIONS);
    println!("{}", pass_or_fail(left_spaceship_simulation.get_generation() == LEFT_SPACESHIP_CRASHED));

    print!("Right Spaceship Crashes: ");
    let mut right_spaceship_simulation = Simulation::new_finite_plane(SPACESHIP_SIMULATION_SIZE, SPACESHIP_SIMULATION_SIZE, RIGHT_SPACESHIP_SEED.to_string());
    right_spaceship_simulation.simulate_generations(NUMBER_OF_SPACESHIP_GENERATIONS);
    println!("{}", pass_or_fail(right_spaceship_simulation.get_generation() == RIGHT_SPACESHIP_CRASHED));
    println!()
}

pub(crate) fn test_finite_spheroid() {
    println!("Testing Finite Spheroid:");

    print!("Up Spaceship Wraps: ");
    let mut up_spaceship_simulation = Simulation::new_finite_spheroid(SPACESHIP_SIMULATION_SIZE, SPACESHIP_SIMULATION_SIZE, UP_SPACESHIP_SEED.to_string());
    up_spaceship_simulation.simulate_generations(NUMBER_OF_SPACESHIP_GENERATIONS);
    println!("{}", pass_or_fail(up_spaceship_simulation.get_generation() == UP_SPACESHIP_WRAPPED));

    print!("Down Spaceship Wraps: ");
    let mut down_spaceship_simulation = Simulation::new_finite_spheroid(SPACESHIP_SIMULATION_SIZE, SPACESHIP_SIMULATION_SIZE, DOWN_SPACESHIP_SEED.to_string());
    down_spaceship_simulation.simulate_generations(NUMBER_OF_SPACESHIP_GENERATIONS);
    println!("{}", pass_or_fail(down_spaceship_simulation.get_generation() == DOWN_SPACESHIP_WRAPPED));

    print!("Left Spaceship Wraps: ");
    let mut left_spaceship_simulation = Simulation::new_finite_spheroid(SPACESHIP_SIMULATION_SIZE, SPACESHIP_SIMULATION_SIZE, LEFT_SPACESHIP_SEED.to_string());
    left_spaceship_simulation.simulate_generations(NUMBER_OF_SPACESHIP_GENERATIONS);
    println!("{}", pass_or_fail(left_spaceship_simulation.get_generation() == LEFT_SPACESHIP_WRAPPED));

    print!("Right Spaceship Wraps: ");
    let mut right_spaceship_simulation = Simulation::new_finite_spheroid(SPACESHIP_SIMULATION_SIZE, SPACESHIP_SIMULATION_SIZE, RIGHT_SPACESHIP_SEED.to_string());
    right_spaceship_simulation.simulate_generations(NUMBER_OF_SPACESHIP_GENERATIONS);
    println!("{}", pass_or_fail(right_spaceship_simulation.get_generation() == RIGHT_SPACESHIP_WRAPPED));
    println!()
}

pub(crate) fn test_finite_vertical_loop() {
    println!("Testing Finite Vertical Loop:");

    print!("Up Spaceship Wraps: ");
    let mut up_spaceship_simulation = Simulation::new_finite_vertical_loop(SPACESHIP_SIMULATION_SIZE, SPACESHIP_SIMULATION_SIZE, UP_SPACESHIP_SEED.to_string());
    up_spaceship_simulation.simulate_generations(NUMBER_OF_SPACESHIP_GENERATIONS);
    println!("{}", pass_or_fail(up_spaceship_simulation.get_generation() == UP_SPACESHIP_WRAPPED));

    print!("Down Spaceship Wraps: ");
    let mut down_spaceship_simulation = Simulation::new_finite_vertical_loop(SPACESHIP_SIMULATION_SIZE, SPACESHIP_SIMULATION_SIZE, DOWN_SPACESHIP_SEED.to_string());
    down_spaceship_simulation.simulate_generations(NUMBER_OF_SPACESHIP_GENERATIONS);
    println!("{}", pass_or_fail(down_spaceship_simulation.get_generation() == DOWN_SPACESHIP_WRAPPED));

    print!("Left Spaceship Crashes: ");
    let mut left_spaceship_simulation = Simulation::new_finite_vertical_loop(SPACESHIP_SIMULATION_SIZE, SPACESHIP_SIMULATION_SIZE, LEFT_SPACESHIP_SEED.to_string());
    left_spaceship_simulation.simulate_generations(NUMBER_OF_SPACESHIP_GENERATIONS);
    println!("{}", pass_or_fail(left_spaceship_simulation.get_generation() == LEFT_SPACESHIP_CRASHED));

    print!("Right Spaceship Crashes: ");
    let mut right_spaceship_simulation = Simulation::new_finite_vertical_loop(SPACESHIP_SIMULATION_SIZE, SPACESHIP_SIMULATION_SIZE, RIGHT_SPACESHIP_SEED.to_string());
    right_spaceship_simulation.simulate_generations(NUMBER_OF_SPACESHIP_GENERATIONS);
    println!("{}", pass_or_fail(right_spaceship_simulation.get_generation() == RIGHT_SPACESHIP_CRASHED));
    println!()
}

pub(crate) fn test_finite_horizontal_loop() {
    println!("Testing Finite Horizontal Loop:");

    print!("Up Spaceship Crashes: ");
    let mut up_spaceship_simulation = Simulation::new_finite_horizontal_loop(SPACESHIP_SIMULATION_SIZE, SPACESHIP_SIMULATION_SIZE, UP_SPACESHIP_SEED.to_string());
    up_spaceship_simulation.simulate_generations(NUMBER_OF_SPACESHIP_GENERATIONS);
    println!("{}", pass_or_fail(up_spaceship_simulation.get_generation() == UP_SPACESHIP_CRASHED));

    print!("Down Spaceship Crashes: ");
    let mut down_spaceship_simulation = Simulation::new_finite_horizontal_loop(SPACESHIP_SIMULATION_SIZE, SPACESHIP_SIMULATION_SIZE, DOWN_SPACESHIP_SEED.to_string());
    down_spaceship_simulation.simulate_generations(NUMBER_OF_SPACESHIP_GENERATIONS);
    println!("{}", pass_or_fail(down_spaceship_simulation.get_generation() == DOWN_SPACESHIP_CRASHED));

    print!("Left Spaceship Wraps: ");
    let mut left_spaceship_simulation = Simulation::new_finite_horizontal_loop(SPACESHIP_SIMULATION_SIZE, SPACESHIP_SIMULATION_SIZE, LEFT_SPACESHIP_SEED.to_string());
    left_spaceship_simulation.simulate_generations(NUMBER_OF_SPACESHIP_GENERATIONS);
    println!("{}", pass_or_fail(left_spaceship_simulation.get_generation() == LEFT_SPACESHIP_WRAPPED));

    print!("Right Spaceship Wraps: ");
    let mut right_spaceship_simulation = Simulation::new_finite_horizontal_loop(SPACESHIP_SIMULATION_SIZE, SPACESHIP_SIMULATION_SIZE, RIGHT_SPACESHIP_SEED.to_string());
    right_spaceship_simulation.simulate_generations(NUMBER_OF_SPACESHIP_GENERATIONS);
    println!("{}", pass_or_fail(right_spaceship_simulation.get_generation() == RIGHT_SPACESHIP_WRAPPED));
    println!()
}

pub(crate) fn test_infinite() {
    test_infinite_plane();
    test_infinite_vertical_strip();
    test_infinite_horizontal_strip();
    test_infinite_vertical_cylinder();
    test_infinite_horizontal_cylinder();
}

pub(crate) fn test_infinite_plane() {}

pub(crate) fn test_infinite_vertical_strip() {}

pub(crate) fn test_infinite_horizontal_strip() {}

pub(crate) fn test_infinite_vertical_cylinder() {}

pub(crate) fn test_infinite_horizontal_cylinder() {}