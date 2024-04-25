use game_of_life::simulation::*;

fn main() {
    let mut ball = SimulationBuilder::new()
        .rows(9)
        .columns(9)
        .surface_type(SurfaceType::Ball)
        .seed("000000000000000000000111000000100100000100000000100000000010100000000000000000000".to_string())
        .build()
        .unwrap();

    for _ in 0..=11 {
        ball.print_current_generation();
        ball.simulate_generation();
    }

    let mut square = SimulationBuilder::new()
        .rows(9)
        .columns(9)
        .surface_type(SurfaceType::Rectangle)
        .seed("000000000000000000000111000000100100000100000000100000000010100000000000000000000".to_string())
        .build()
        .unwrap();

    for _ in 0..=11 {
        square.print_current_generation();
        square.simulate_generation();
    }
}