#![allow(dead_code)]

use game_of_life::simulation::*;

fn main() {
    test_surface_behaviors();
    test_printing();
}

fn passed_or_failed(pass: bool) -> &'static str {
    if pass { "PASSED" } else { "FAILED" }
}

pub(crate) fn print_side_by_side(left_grid: String, left_grid_title: &str, right_grid: String, right_grid_title: &str, rows: u128, columns: u128) {
    println!();
    let full_print_width = 2 * columns + 3;
    let title_spaces = full_print_width - left_grid_title.len() as u128 - right_grid_title.len() as u128;
    println!("{}{}{}", left_grid_title, " ".repeat((title_spaces - 1) as usize), right_grid_title);
    for row in 0..rows {
        let start = (row * columns) as usize;
        let end = start + columns as usize;
        println!("{}   {}", &left_grid[start..end], &right_grid[start..end]);
    }
    println!();
}

pub(crate) fn test_printing() {
    println!("~~~TESTING PRINTING~~~");
    test_surface_printing(15, 3, |rows, cols|
        SimulationBuilder::new()
            .rows(rows)
            .columns(cols)
            .surface_type(SurfaceType::Rectangle)
            .build()
            .unwrap(), "Rectangle");
    test_surface_printing(15, 3, |rows, cols|
        SimulationBuilder::new()
            .rows(rows)
            .columns(cols)
            .surface_type(SurfaceType::Ball)
            .build()
            .unwrap(), "Ball");
    test_surface_printing(15, 3, |rows, cols|
        SimulationBuilder::new()
            .rows(rows)
            .columns(cols)
            .surface_type(SurfaceType::VerticalLoop)
            .build()
            .unwrap(), "Vertical Loop");
    test_surface_printing(15, 3, |rows, cols|
        SimulationBuilder::new()
            .rows(rows)
            .columns(cols)
            .surface_type(SurfaceType::HorizontalLoop)
            .build()
            .unwrap(), "Horizontal Loop");
}

fn test_surface_printing<S>(simulation_size: u16, generation_iterations: u128, new_simulation: S, surface_name: &str)
    where
        S: Fn(u16, u16) -> Simulation,
{
    println!("Testing {}:", surface_name);
    let mut simulation = new_simulation(simulation_size, simulation_size);
    println!("{}\n", simulation.as_seed());
    simulation.simulate_generations(generation_iterations);
    println!("{}\n", simulation);
}

pub(crate) fn test_surface_behaviors() {
    println!("~~~TESTING SURFACE BEHAVIOR~~~");
    let simulation_size = 9;
    let generation_iterations = 15;

    let up_seed = "---------------------***------*--*-----*--------*---------*-*--------------------";
    let down_seed = "----------------------*-*-----*--------*--------*--*-----***---------------------";
    let left_seed = "---------------------*--*----*--------*---*----****------------------------------";
    let right_seed = "--------------------*--*---------*----*---*-----****-----------------------------";

    let up_spaceship_crashed = "------------------------*---------**------**-------------------------------------";
    let down_spaceship_crashed = "------------------------------------------**--------**------*--------------------";
    let left_spaceship_crashed = "---*--------**------*-*----------------------------------------------------------";
    let right_spaceship_crashed = "-----*-------**-------*-*--------------------------------------------------------";

    let up_spaceship_wrapped = "-------------------------------*-------***------*-**------***------**------------";
    let down_spaceship_wrapped = "-------------**-------***-----*-**-----***-------*-------------------------------";
    let left_spaceship_wrapped = "-----------------------**------****----**-**-----**------------------------------";
    let right_spaceship_wrapped = "--------------------**------****-----**-**------**-------------------------------";

    test_surface_behavior(simulation_size, generation_iterations, |rows, cols, seed|
        SimulationBuilder::new()
            .rows(rows)
            .columns(cols)
            .surface_type(SurfaceType::Rectangle)
            .seed(seed)
            .build()
            .unwrap(), "Rectangle",
                          up_seed, up_spaceship_crashed,
                          down_seed, down_spaceship_crashed,
                          left_seed, left_spaceship_crashed,
                          right_seed, right_spaceship_crashed);
    test_surface_behavior(simulation_size, generation_iterations, |rows, cols, seed|
        SimulationBuilder::new()
            .rows(rows)
            .columns(cols)
            .surface_type(SurfaceType::Ball)
            .seed(seed)
            .build()
            .unwrap(), "Ball",
                          up_seed, up_spaceship_wrapped,
                          down_seed, down_spaceship_wrapped,
                          left_seed, left_spaceship_wrapped,
                          right_seed, right_spaceship_wrapped);
    test_surface_behavior(simulation_size, generation_iterations, |rows, cols, seed|
        SimulationBuilder::new()
            .rows(rows)
            .columns(cols)
            .surface_type(SurfaceType::VerticalLoop)
            .seed(seed)
            .build()
            .unwrap(), "Vertical Loop",
                          up_seed, up_spaceship_wrapped,
                          down_seed, down_spaceship_wrapped,
                          left_seed, left_spaceship_crashed,
                          right_seed, right_spaceship_crashed);
    test_surface_behavior(simulation_size, generation_iterations, |rows, cols, seed|
        SimulationBuilder::new()
            .rows(rows)
            .columns(cols)
            .surface_type(SurfaceType::HorizontalLoop)
            .seed(seed)
            .build()
            .unwrap(), "Horizontal Loop",
                          up_seed, up_spaceship_crashed,
                          down_seed, down_spaceship_crashed,
                          left_seed, left_spaceship_wrapped,
                          right_seed, right_spaceship_wrapped);
}

fn test_surface_behavior<S>(simulation_size: u16, generation_iterations: u128, new_simulation: S, surface_name: &str,
                            up_seed: &str, up_result: &str,
                            down_seed: &str, down_result: &str,
                            left_seed: &str, left_result: &str,
                            right_seed: &str, right_result: &str)
    where
        S: Fn(u16, u16, String) -> Simulation,
{
    println!("Testing {}:", surface_name);

    let test_case = |seed: &str, expected: &str, direction: &str| {
        print!("{} Spaceship: ", direction);
        let mut simulation = new_simulation(simulation_size, simulation_size, seed.to_string());
        simulation.simulate_generations(generation_iterations);
        let simulation_is_expected = simulation.generation_string() == expected;
        println!("{}", passed_or_failed(simulation_is_expected));
        if !simulation_is_expected {
            print_side_by_side(simulation.generation_string(), "RESULT",
                               String::from(expected), "EXPECTED", simulation.rows as u128, simulation.columns as u128);
        }
    };

    test_case(up_seed, up_result, "Up");
    test_case(down_seed, down_result, "Down");
    test_case(left_seed, left_result, "Left");
    test_case(right_seed, right_result, "Right");
    println!();
}