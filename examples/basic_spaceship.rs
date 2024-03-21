use game_of_life::simulation::*;

fn main() {
    let mut ball = Simulation::new_ball(9u8, 9u8, String::from("000000000000000000000111000000100100000100000000100000000010100000000000000000000"));
    for _ in 0..=11 {
        ball.print_current_generation();
        ball.simulate_generation();
    }

    let mut square = Simulation::new_square(9u8, String::from("000000000000000000000111000000100100000100000000100000000010100000000000000000000"));
    for _ in 0..=11 {
        square.print_current_generation();
        square.simulate_generation();
    }
}