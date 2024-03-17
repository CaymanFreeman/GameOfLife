use crate::simulation::Simulation;

fn passed_or_failed(pass: bool) -> &'static str {
    if pass { "PASSED" } else { "FAILED" }
}

pub(crate) fn print_side_by_side(left_grid: String, left_grid_title: &str, right_grid: String, right_grid_title: &str, rows: i32, columns: i32) {
    println!();
    let full_print_width = 2 * columns + 3;
    let title_spaces = (full_print_width - left_grid_title.len() as i32 - right_grid_title.len() as i32) as usize;
    println!("{}{}{}", left_grid_title, " ".repeat(title_spaces - 1), right_grid_title);
    for row in 0..rows {
        let start = (row * columns) as usize;
        let end = start + columns as usize;
        println!("{}   {}", &left_grid[start..end], &right_grid[start..end]);
    }
    println!();
}

pub(crate) fn test_printing() {
    println!("~~~TESTING PRINTING~~~");
    let simulation_size = 15;
    let generation_iterations = 3;

    test_surface_printing(simulation_size, generation_iterations, Simulation::new_plane_rand, "Plane");
    test_surface_printing(simulation_size, generation_iterations, Simulation::new_spheroid_rand, "Spheroid");
    test_surface_printing(simulation_size, generation_iterations, Simulation::new_vertical_loop_rand, "Vertical Loop");
    test_surface_printing(simulation_size, generation_iterations, Simulation::new_horizontal_loop_rand, "Horizontal Loop");
}

fn test_surface_printing<F>(simulation_size: i32, generation_iterations: u128, new_simulation: F, surface_name: &str)
    where
        F: Fn(i32, i32) -> Simulation,
{
    println!("Testing {}:", surface_name);
    let mut simulation = new_simulation(simulation_size, simulation_size);
    simulation.print_seed_generation(Some(true));
    println!();
    simulation.simulate_generations(generation_iterations);
    simulation.print_current_generation();
    println!();
}

pub(crate) fn test_surface_behavior() {
    println!("~~~TESTING SURFACE BEHAVIOR~~~");
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

    test_surface(simulation_size, generation_iterations, Simulation::new_plane, "Plane",
                 up_seed, up_spaceship_crashed,
                 down_seed, down_spaceship_crashed,
                 left_seed, left_spaceship_crashed,
                 right_seed, right_spaceship_crashed);
    test_surface(simulation_size, generation_iterations, Simulation::new_spheroid, "Spheroid",
                 up_seed, up_spaceship_wrapped,
                 down_seed, down_spaceship_wrapped,
                 left_seed, left_spaceship_wrapped,
                 right_seed, right_spaceship_wrapped);
    test_surface(simulation_size, generation_iterations, Simulation::new_vertical_loop, "Vertical Loop",
                 up_seed, up_spaceship_wrapped,
                 down_seed, down_spaceship_wrapped,
                 left_seed, left_spaceship_crashed,
                 right_seed, right_spaceship_crashed);
    test_surface(simulation_size, generation_iterations, Simulation::new_horizontal_loop, "Horizontal Loop",
                 up_seed, up_spaceship_crashed,
                 down_seed, down_spaceship_crashed,
                 left_seed, left_spaceship_wrapped,
                 right_seed, right_spaceship_wrapped);
}

fn test_surface<F>(simulation_size: i32, generation_iterations: u128, new_simulation: F, surface_name: &str,
                   up_seed: &str, up_result: &str,
                   down_seed: &str, down_result: &str,
                   left_seed: &str, left_result: &str,
                   right_seed: &str, right_result: &str)
    where
        F: Fn(i32, i32, String) -> Simulation,
{
    println!("Testing {}:", surface_name);

    let test_case = |seed: &str, expected: &str, direction: &str| {
        print!("{} Spaceship: ", direction);
        let mut simulation = new_simulation(simulation_size, simulation_size, seed.to_string());
        simulation.simulate_generations(generation_iterations);
        let simulation_is_expected = simulation.get_generation_string() == expected;
        println!("{}", passed_or_failed(simulation_is_expected));
        if !simulation_is_expected {
            print_side_by_side(simulation.get_generation_string(), "RESULT", String::from(expected), "EXPECTED", simulation.rows, simulation.columns);
        }
    };

    test_case(up_seed, up_result, "Up");
    test_case(down_seed, down_result, "Down");
    test_case(left_seed, left_result, "Left");
    test_case(right_seed, right_result, "Right");
    println!();
}