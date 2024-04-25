use game_of_life::simulation::*;

fn main() {
    let rows: u16 = 10;
    let columns: u16 = 35;
    let mut fittest_seed: String = String::new();
    let mut fittest_generations: u128 = 0;
    loop {
        println!("The fittest seed so far lasted for {} generations: {}", fittest_generations, fittest_seed);
        println!("Creating new simulation...\n");
        let mut active_simulation = SimulationBuilder::new()
            .maximum_saves(1000000)
            .rows(rows)
            .columns(columns)
            .surface_type(SurfaceType::Rectangle)
            .seed(random_seed_string(rows, columns))
            .build()
            .unwrap();

        let mut still_or_period = false;

        while !still_or_period {
            active_simulation.simulate_generation();
            still_or_period = active_simulation.is_still();
            for i in 2..=active_simulation.maximum_saves {
                still_or_period = still_or_period || active_simulation.is_period(i as usize);
                if still_or_period {
                    break;
                }
            }
        }
        if active_simulation.is_still() {
            println!("\nSimulation has become still, it lasted for {} generations.", active_simulation.generation_iteration - 1);
        } else {
            let periods = 2..=active_simulation.maximum_saves;
            for period in periods {
                if active_simulation.is_period(period as usize) {
                    println!("\nSimulation has a period of {}, it lasted for {} generations.", period, active_simulation.generation_iteration - 1);
                    break;
                }
            }
        }
        if active_simulation.clone().generation_iteration - 1 > active_simulation.maximum_saves {
            println!("\nThe maximum generation limit of {} has been recorded, stopping all simulations.", active_simulation.maximum_saves);
            break;
        }
        if active_simulation.clone().generation_iteration - 1 > fittest_generations {
            fittest_generations = active_simulation.generation_iteration - 1;
            fittest_seed = active_simulation.seed;
        }
    }
}