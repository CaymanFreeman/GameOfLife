use std::collections::HashSet;
use num_traits::{ToPrimitive};
use crate::cell::Cell;
use crate::cell::CellState::{Alive, Dead};
use crate::seeds::{random_seed_string, seed_string_to_generation};
use crate::simulation::SurfaceType::*;

#[derive(Clone)]
pub enum SurfaceType {
    Ball,
    HorizontalLoop,
    VerticalLoop,
    Rectangle,
}

pub struct Simulation {
    pub seed: String,
    pub surface_type: SurfaceType,
    pub rows: u16,
    pub columns: u16,
    pub generation: HashSet<Cell>,
    pub generation_iteration: u128,
    pub save_history: Vec<HashSet<Cell>>,
    pub maximum_saves: u16,
    pub simulation_cooldown: u16,
}

pub struct SimulationBuilder {
    rows: Option<u16>,
    columns: Option<u16>,
    surface_type: SurfaceType,
    seed: Option<String>,
    maximum_saves: u16,
    simulation_cooldown: u16,
}

impl Default for SimulationBuilder {
    fn default() -> Self {
        Self {
            rows: Some(10),
            columns: Some(10),
            surface_type: Rectangle,
            seed: None,
            maximum_saves: 100,
            simulation_cooldown: 1,
        }
    }
}

impl SimulationBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn rows(mut self, rows: u16) -> Self {
        self.rows = Some(rows);
        self
    }

    pub fn columns(mut self, columns: u16) -> Self {
        self.columns = Some(columns);
        self
    }

    pub fn surface_type(mut self, surface_type: SurfaceType) -> Self {
        self.surface_type = surface_type;
        self
    }

    pub fn seed(mut self, seed: String) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn maximum_saves(mut self, maximum_saves: u16) -> Self {
        self.maximum_saves = maximum_saves;
        self
    }

    pub fn simulation_cooldown(mut self, simulation_cooldown: u16) -> Self {
        self.simulation_cooldown = simulation_cooldown;
        self
    }

    pub fn build(self) -> Result<Simulation, String> {
        let (rows, columns, seed) = match (self.rows, self.columns, self.seed) {
            (Some(rows), Some(columns), Some(seed)) => (rows, columns, seed),
            (Some(rows), Some(columns), None) => (rows, columns, random_seed_string(rows, columns)),
            (Some(rows), None, Some(seed)) => {
                let seed_length = seed.len() as u16;
                if seed_length % rows == 0 {
                    (rows, seed_length / rows, seed)
                } else {
                    return Err(format!("The provided seed of \"{}\", must be divisible by the number of rows: {}", seed, rows));
                }
            }
            (None, Some(columns), Some(seed)) => {
                let seed_length = seed.len() as u16;
                if seed_length % columns == 0 {
                    (seed_length / columns, columns, seed)
                } else {
                    return Err(format!("The provided seed of \"{}\", must be divisible by the number of columns: {}", seed, columns));
                }
            }
            (None, None, Some(seed)) => {
                let seed_length = seed.len() as u16;
                if let Some(sqrt) = (seed_length as f32).sqrt().to_u16() {
                    (sqrt, sqrt, seed)
                } else {
                    return Err(format!("The provided seed of \"{}\", must be of a square size (has an integer square root)", seed));
                }
            }
            (Some(_), None, None) | (None, Some(_), None) => {
                return Err("Both rows and columns must be provided if no seed is provided".to_string());
            }
            (None, None, None) => {
                return Err("One of the following must be provided: rows, columns, or seed".to_string());
            }
        };

        let generation = seed_string_to_generation(seed.clone(), columns).unwrap();

        Ok(Simulation {
            seed,
            surface_type: self.surface_type,
            rows,
            columns,
            generation,
            generation_iteration: 0,
            save_history: Vec::new(),
            maximum_saves: self.maximum_saves,
            simulation_cooldown,
        })
    }
}

impl Simulation {

    pub fn get_seed_string(&self) -> String { self.seed.clone() }

    pub fn get_generation_string(&self) -> String {
        let mut current_generation = String::new();
        let mut row = 0;
        while row < self.rows {
            let mut column = 0;
            while column < self.columns {
                current_generation.push(self.get_cell(row.clone(), column.clone()).to_seed_value());
                column = column + 1;
            }
            row = row + 1;
        }
        current_generation
    }

    pub fn print_seed_generation(&self, print_with_grid: Option<bool>) {
        let print_with_grid = print_with_grid.unwrap_or(false);
        println!("SEED: {}", self.seed);
        if print_with_grid {
            let first_iteration = Self::from_seed_generation(self);
            let mut row = 0;
            while row < first_iteration.rows {
                let mut column = 0;
                while column < first_iteration.columns {
                    print!("{}", first_iteration.get_cell(row.clone(), column.clone()).to_display());
                    column = column + 1;
                }
                print!("\n");
                row = row + 1;
            }
        }
    }

    pub fn print_current_generation(&self) {
        if self.generation_iteration == 0 {
            println!("SEED GENERATION");
        } else {
            println!("{}", self.generation_iteration);
        }
        let mut row = 0;
        while row < self.rows {
            let mut column = 0;
            while column < self.columns {
                print!("{}", self.get_cell(row.clone(), column.clone()).to_display());
                column = column + 1;
            }
            print!("\n");
            row = row + 1;
        }
    }

    fn get_cell(&self, row: u16, column: u16) -> Cell {
        let mut cell = Cell::new_alive(row, column);
        if !self.generation.contains(&cell) {
            cell.state = Dead;
        }
        return cell
    }

    // Behold, efficiency
    fn get_alive_neighbors(&self, cell: Cell) -> u8 {
        let origin_row = cell.row;
        let origin_column = cell.column;
        let mut wrapping_vertically = false;
        let mut wrapping_horizontally = false;
        let mut bounded_vertically = false;
        let mut bounded_horizontally = false;
        match self.surface_type.clone() {
            Ball => {
                wrapping_vertically = true;
                wrapping_horizontally = true;
            }
            HorizontalLoop => {
                wrapping_horizontally = true;
                bounded_vertically = true;
            }
            VerticalLoop => {
                wrapping_vertically = true;
                bounded_horizontally = true;
            }
            Rectangle => {
                bounded_vertically = true;
                bounded_horizontally = true;
            }
        }

        let on_top_edge = origin_row == 0;
        let on_bottom_edge = origin_row == self.rows.clone() - 1;
        let on_left_edge = origin_column == 0;
        let on_right_edge = origin_column == self.columns.clone() - 1;

        let top_left_is_alive = {
            let result = (|| {
                if on_top_edge && bounded_vertically {
                    return false;
                }
                if on_left_edge && bounded_horizontally {
                    return false;
                }
                let neighbor_row;
                let neighbor_column;
                if on_top_edge && wrapping_vertically {
                    neighbor_row = self.rows.clone() - 1
                } else {
                    neighbor_row = origin_row.clone() - 1
                }
                if on_left_edge && wrapping_horizontally {
                    neighbor_column = self.columns.clone() - 1
                } else {
                    neighbor_column = origin_column.clone() - 1
                }
                self.get_cell(neighbor_row, neighbor_column).is_alive()
            })();
            result
        };
        let top_center_is_alive = {
            let result = (|| {
                if on_top_edge && bounded_vertically {
                    return false;
                }
                let neighbor_row;
                if on_top_edge && wrapping_vertically {
                    neighbor_row = self.rows.clone() - 1
                } else {
                    neighbor_row = origin_row.clone() - 1
                }
                self.get_cell(neighbor_row, origin_column.clone()).is_alive()
            })();
            result
        };
        let top_right_is_alive = {
            let result = (|| {
                if on_top_edge && bounded_vertically {
                    return false;
                }
                if on_right_edge && bounded_horizontally {
                    return false;
                }
                let neighbor_row;
                let neighbor_column;
                if on_top_edge && wrapping_vertically {
                    neighbor_row = self.rows.clone() - 1
                } else {
                    neighbor_row = origin_row.clone() - 1
                }
                if on_right_edge && wrapping_horizontally {
                    neighbor_column = 0;
                } else {
                    neighbor_column = origin_column.clone() + 1
                }
                self.get_cell(neighbor_row, neighbor_column).is_alive()
            })();
            result
        };
        let middle_left_is_alive = {
            let result = (|| {
                if on_left_edge && bounded_horizontally {
                    return false;
                }
                let neighbor_column;
                if on_left_edge && wrapping_horizontally {
                    neighbor_column = self.columns.clone() - 1
                } else {
                    neighbor_column = origin_column.clone() - 1
                }
                self.get_cell(origin_row.clone(), neighbor_column).is_alive()
            })();
            result
        };
        let middle_right_is_alive = {
            let result = (|| {
                if on_right_edge && bounded_horizontally {
                    return false;
                }
                let neighbor_column;
                if on_right_edge && wrapping_horizontally {
                    neighbor_column = 0;
                } else {
                    neighbor_column = origin_column.clone() + 1
                }
                self.get_cell(origin_row.clone(), neighbor_column).is_alive()
            })();
            result
        };
        let bottom_left_is_alive = {
            let result = (|| {
                if on_left_edge && bounded_horizontally {
                    return false;
                }
                if on_bottom_edge && bounded_vertically {
                    return false;
                }
                let neighbor_row;
                let neighbor_column;
                if on_bottom_edge && wrapping_vertically {
                    neighbor_row = 0;
                } else {
                    neighbor_row = origin_row.clone() + 1
                }
                if on_left_edge && wrapping_horizontally {
                    neighbor_column = self.columns.clone() - 1
                } else {
                    neighbor_column = origin_column.clone() - 1
                }
                self.get_cell(neighbor_row, neighbor_column).is_alive()
            })();
            result
        };
        let bottom_center_is_alive = {
            let result = (|| {
                if on_bottom_edge && bounded_vertically {
                    return false;
                }
                let neighbor_row;
                if on_bottom_edge && wrapping_vertically {
                    neighbor_row = 0;
                } else {
                    neighbor_row = origin_row.clone() + 1
                }
                self.get_cell(neighbor_row, origin_column.clone()).is_alive()
            })();
            result
        };
        let bottom_right_is_alive = {
            let result = (|| {
                if on_bottom_edge && bounded_vertically {
                    return false;
                }
                if on_right_edge && bounded_horizontally {
                    return false;
                }
                let neighbor_row;
                let neighbor_column;
                if on_bottom_edge && wrapping_vertically {
                    neighbor_row = 0;
                } else {
                    neighbor_row = origin_row.clone() + 1
                }
                if on_right_edge && wrapping_horizontally {
                    neighbor_column = 0;
                } else {
                    neighbor_column = origin_column.clone() + 1
                }
                self.get_cell(neighbor_row, neighbor_column).is_alive()
            })();
            result
        };

        let mut count = 0;
        if top_left_is_alive { count += 1 }
        if top_center_is_alive { count += 1 }
        if top_right_is_alive { count += 1 }
        if middle_left_is_alive { count += 1 }
        if middle_right_is_alive { count += 1 }
        if bottom_left_is_alive { count += 1 }
        if bottom_center_is_alive { count += 1 }
        if bottom_right_is_alive { count += 1 }
        count
    }

    pub fn save_generation(&mut self) {
        if self.save_history.len() as u16 == self.maximum_saves {
            self.save_history.remove(0);
        }
        self.save_history.push(self.generation.clone());
    }

    pub fn rollback_generations(&mut self, iterations: u128) {
        if iterations == 0 {
            return;
        }
        for _ in 0..iterations {
            if let Some(previous_generation) = self.save_history.pop() {
                self.generation = previous_generation;
                self.generation_iteration -= 1;
            } else {
                break;
            }
        }
    }

    pub fn rollback_generation(&mut self) {
        self.rollback_generations(1)
    }

    pub fn simulate_generations(&mut self, iterations: u128) {
        if iterations == 0 {
            return
        }
        self.save_generation();
        for _ in 0..iterations {
            let mut new_generation = self.generation.clone();
            let mut row = 0;
            while row < self.rows {
                let mut column = 0;
                while column < self.columns {
                    let mut cell = self.get_cell(row.clone(), column.clone());
                    let alive_neighbors = self.get_alive_neighbors(cell.clone());
                    let cell_alive = cell.is_alive();
                    if cell_alive {
                        if alive_neighbors < 2 || alive_neighbors > 3 {
                            new_generation.remove(&cell);
                        }
                    } else {
                        if alive_neighbors == 3 {
                            cell.state = Alive;
                            new_generation.insert(cell);
                        }
                    }
                    column = column + 1;
                }
                row = row + 1;
            }
            self.generation = new_generation;
            self.generation_iteration += 1;
        }
    }

    pub fn simulate_generation(&mut self) {
        self.simulate_generations(1)
    }

    pub fn clone(&self) -> Simulation {
        Simulation {
            seed: self.seed.clone(),
            surface_type: self.surface_type.clone(),
            rows: self.rows.clone(),
            columns: self.columns.clone(),
            generation: self.generation.clone(),
            generation_iteration: self.generation_iteration,
            save_history: self.save_history.clone(),
            maximum_saves: self.maximum_saves,
        }
    }

    pub fn from_seed_generation(simulation: &Simulation) -> Simulation {
        Simulation {
            seed: simulation.seed.clone(),
            surface_type: simulation.surface_type.clone(),
            rows: simulation.rows.clone(),
            columns: simulation.columns.clone(),
            generation: seed_string_to_generation(simulation.seed.clone(), simulation.columns.clone()).unwrap(),
            generation_iteration: 0,
            save_history: simulation.save_history.clone(),
            maximum_saves: simulation.maximum_saves,
        }
    }
}