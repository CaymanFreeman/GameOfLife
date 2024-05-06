use std::thread::sleep;
use std::time::Duration;
use game_of_life::simulation::*;

fn main() {
    let rows: u16 = 10;
    let columns: u16 = 15;
    let mut active_simulation: Simulation = SimulationBuilder::new()
        .maximum_saves(1000000)
        .rows(rows)
        .columns(columns)
        .surface_type(SurfaceType::Rectangle)
        .has_display(true)
        .window_title("Basic Display Simulation")
        .cell_size(50)
        .build()
        .unwrap();

    for _i in 0..5 {
        active_simulation.simulate_generation();
        sleep(Duration::from_secs(3));
    }
}