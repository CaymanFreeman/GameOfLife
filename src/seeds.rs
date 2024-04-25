use std::collections::HashSet;
use crate::cell::Cell;
use rand::{Rng, thread_rng};

pub fn seed_string_to_generation(seed: String, columns: u16) -> Result<HashSet<Cell>, String> {
    let mut generation = HashSet::new();
    let values: Vec<char> = seed.chars().collect();
    for i in 0..values.len() {
        let index = i as u16;
        let row_index = index.clone() / columns.clone();
        let column_index = index % columns.clone();
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

pub fn random_seed_string(rows: u16, columns: u16) -> String {
    let length = rows * columns;
    let mut seed = String::new();
    let mut rng = thread_rng();
    let mut i = 0;
    while i < length {
        let random_number = rng.gen_range('0'..='1');
        seed.push(random_number);
        i = i + 1;
    }
    seed
}