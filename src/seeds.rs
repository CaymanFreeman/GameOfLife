use std::collections::HashSet;
use crate::cell::Cell;
use rand::{Rng, thread_rng};

pub(crate) fn seed_string_to_generation(seed: String, columns: i32) -> Result<HashSet<Cell>, String> {
    let mut generation = HashSet::new();
    let values: Vec<char> = seed.chars().collect();
    for i in 0..values.len() {
        let index = i as i32;
        let row_index = index / columns;
        let column_index = index % columns;
        let value = values.get(i).unwrap().clone();
        match value {
            '1' => {
                generation.insert(Cell::new_alive(row_index, column_index));
            },
            '0' => {},
            _ => return Err(format!("Unexpected seed character: {}", value)),
        };
    }
    Ok(generation)
}

pub(crate) fn random_seed_string(rows: i32, columns: i32) -> String {
    let length = rows * columns;
    let mut seed = String::new();
    let mut rng = thread_rng();
    for _ in 0..length {
        let random_number = rng.gen_range('0'..='1');
        seed.push(random_number);
    }
    seed
}