use game_of_life::simulation::*;

fn main() {
    let mut ball = SimulationBuilder::new()
        .rows(9)
        .columns(9)
        .surface_type(SurfaceType::Ball)
        .seed("---------------------***------*--*-----*--------*---------*-*--------------------".to_string())
        .build()
        .unwrap();

    for _ in 0..=11 {
        println!("{}", ball);
        ball.simulate_generation();
    }

    let mut square = SimulationBuilder::new()
        .rows(9)
        .columns(9)
        .surface_type(SurfaceType::Rectangle)
        .seed("---------------------***------*--*-----*--------*---------*-*--------------------".to_string())
        .build()
        .unwrap();

    for _ in 0..=11 {
        println!("{}", square);
        square.simulate_generation();
    }
}