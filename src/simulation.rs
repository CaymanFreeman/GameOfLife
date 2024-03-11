use crate::cell::Cell;
use crate::cell::CellState::{Alive, Dead};
use crate::simulation::SurfaceType::*;
use rand::{Rng, thread_rng};

#[derive(Clone)]
pub(crate) enum SurfaceType {
    Spheroid,             // Wrapping: left-right & up-down,  Bounds: None
    Plane,                // Wrapping: None,                  Bounds: left-right & up-down
    HorizontalLoop,       // Wrapping: left-right,            Bounds: up-down
    VerticalLoop,         // Wrapping: up-down,               Bounds: left-right
}

pub(crate) struct Simulation {
    pub(crate) seed: String,
    pub(crate) surface_type: SurfaceType,
    pub(crate) rows: i32,
    pub(crate) columns: i32,
    pub(crate) generation: Vec<Cell>,
    pub(crate) generation_iteration: u128,
}

impl Simulation {

    pub(crate) fn seed_string_to_generation(seed: String, columns: i32) -> Result<Vec<Cell>, String> {
        let mut cell_grid = Vec::new();
        let values: Vec<char> = seed.chars().collect();
        for i in 0..values.len() {
            let index = i as i32;
            let row_index = index / columns;
            let column_index = index % columns;
            let value = values.get(i).unwrap().clone();
            let cell = match value {
                '1' => Ok(Cell::new(Alive, row_index, column_index)),
                '0' => Ok(Cell::new(Dead, row_index, column_index)),
                _ => Err(format!("Invalid value (not 1 or 0) in seed: {}", value)),
            }.unwrap();
            cell_grid.push(cell)
        }
        Ok(cell_grid)
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

    pub(crate) fn get_seed_string(&self) -> String { self.seed.clone() }

    pub(crate) fn get_generation_string(&self) -> String {
        let mut current_generation = String::new();
        for row in 0..self.rows {
            for column in 0..self.columns {
                current_generation.push(self.get_cell(row, column).to_seed_value())
            }
        }
        current_generation
    }

    pub(crate) fn print_seed_generation(&self, print_with_grid: Option<bool>) {
        let print_with_grid = print_with_grid.unwrap_or(false);
        println!("SEED: {}", self.seed);
        if print_with_grid {
            let first_iteration = Self::from_as_first(self);
            for row in 0..first_iteration.rows {
                for column in 0..first_iteration.columns {
                    print!("{}", first_iteration.get_cell(row, column).to_display())
                }
                print!("\n")
            }
        }
    }

    pub(crate) fn print_current_generation(&self) {
        if self.generation_iteration == 0 {
            println!("SEED GENERATION");
        } else {
            println!("{}", self.generation_iteration);
        }
        for row in 0..self.rows {
            for column in 0..self.columns {
                print!("{}", self.get_cell(row, column).to_display())
            }
            print!("\n")
        }
    }

    fn wrap_index(index: i32, axis_length: i32) -> i32 {
        if index > -1 {
            index % axis_length
        } else {
            axis_length - (index % axis_length).abs()
        }
    }

    fn out_of_bounds_row(&self, index: i32) -> bool {
        if (index > self.rows - 1) || (index < 0) {
            return true
        }
        return false
    }

    fn out_of_bounds_column(&self, index: i32) -> bool {
        if (index > self.columns - 1) || (index < 0) {
            return true
        }
        return false
    }

    fn get_cell(&self, row: i32, column: i32) -> Cell {
        match self.surface_type.clone() {
            Spheroid => {
                self.generation.iter()
                    .nth((Self::wrap_index(row, self.rows) * self.columns + Self::wrap_index(column, self.columns)) as usize)
                    .unwrap()
                    .clone()
            }
            Plane => {
                if self.out_of_bounds_row(row) || self.out_of_bounds_column(column) {
                    Cell::new(Dead, row, column)
                } else {
                    self.generation.iter()
                        .nth((row * self.columns + column) as usize)
                        .unwrap()
                        .clone()
                }
            }
            HorizontalLoop => {
                if self.out_of_bounds_row(row) {
                    Cell::new(Dead, row, column)
                } else {
                    self.generation.iter()
                        .nth((row * self.columns + Self::wrap_index(column, self.columns)) as usize)
                        .unwrap()
                        .clone()
                }
            }
            VerticalLoop => {
                if self.out_of_bounds_column(column) {
                    Cell::new(Dead, row, column)
                } else {
                    self.generation.iter()
                        .nth((Self::wrap_index(row, self.rows) * self.columns + column) as usize)
                        .unwrap()
                        .clone()
                }
            }
        }
    }

    fn get_alive_neighbors(&self, row: i32, column: i32) -> u8 {
        let mut count = 0;
        if self.get_cell(row - 1, column - 1).is_alive() { count += 1 }
        if self.get_cell(row - 1, column).is_alive() { count += 1 }
        if self.get_cell(row - 1, column + 1).is_alive() { count += 1 }
        if self.get_cell(row, column - 1).is_alive() { count += 1 }

        if self.get_cell(row, column + 1).is_alive() { count += 1 }
        if self.get_cell(row + 1, column - 1).is_alive() { count += 1 }
        if self.get_cell(row + 1, column).is_alive() { count += 1 }
        if self.get_cell(row + 1, column + 1).is_alive() { count += 1 }
        count
    }

    pub(crate) fn simulate_generations(&mut self, iterations: u128) {
        if iterations == 0 {
            return
        }
        for _ in 0..iterations {
            let mut new_generation_grid: Vec<Cell> = self.generation.clone();
            for row in 0..self.rows {
                for col in 0..self.columns {
                    let alive_neighbors = self.get_alive_neighbors(row, col);
                    let cell_index = (row * self.columns + col) as usize;
                    let cell = self.get_cell(row, col);
                    let cell_alive = cell.is_alive();
                    if cell_alive {
                        if alive_neighbors < 2 || alive_neighbors > 3 {
                            new_generation_grid[cell_index].state = Dead
                        }
                    } else if !cell_alive {
                        if alive_neighbors == 3 {
                            new_generation_grid[cell_index].state = Alive
                        }
                    }
                }
            }
            self.generation = new_generation_grid.into_iter().collect();
            self.generation_iteration += 1;
        }
    }

    pub(crate) fn simulate_generation(&mut self) {
        self.simulate_generations(1)
    }

    fn is_perfect_square(number: i32) -> bool {
        let sqrt = (number as f32).sqrt() as i32;
        sqrt * sqrt == number
    }

    pub(crate) fn clone(&self) -> Simulation {
        Simulation {
            seed: self.seed.clone(),
            surface_type: self.surface_type.clone(),
            rows: self.rows,
            columns: self.columns,
            generation: self.generation.clone(),
            generation_iteration: self.generation_iteration,
        }
    }

    pub(crate) fn from(simulation: &Simulation) -> Simulation {
        Simulation {
            seed: simulation.seed.clone(),
            surface_type: simulation.surface_type.clone(),
            rows: simulation.rows,
            columns: simulation.columns,
            generation: simulation.generation.clone(),
            generation_iteration: simulation.generation_iteration,
        }
    }

    pub(crate) fn from_as_first(simulation: &Simulation) -> Simulation {
        Simulation {
            seed: simulation.seed.clone(),
            surface_type: simulation.surface_type.clone(),
            rows: simulation.rows,
            columns: simulation.columns,
            generation: Simulation::seed_string_to_generation(simulation.seed.clone(), simulation.columns).unwrap(),
            generation_iteration: 0,
        }
    }

    pub(crate) fn new(rows_parameter: Option<i32>, columns_parameter: Option<i32>, surface_type: SurfaceType, seed: Option<String>) -> Result<Simulation, String> {
        let mut calculated_seed = String::new();
        let mut rows = rows_parameter.unwrap();
        let mut columns = columns_parameter.unwrap();
        if seed.is_some() {
            calculated_seed = seed.clone().unwrap();
            let seed_length = seed.clone().unwrap().len() as i32;
            if rows_parameter.is_some() && columns_parameter.is_none() {
                if seed_length % rows == 0 {
                    columns = seed_length / rows;
                } else {
                    return Err(format!("The provided seed of \"{}\", must be divisible by the number of rows: {}, if the number of columns is not provided", seed.unwrap(), rows))
                }
            } else if columns_parameter.is_some() && rows_parameter.is_none() {
                if seed_length % columns == 0 {
                    rows = seed_length / columns;
                } else {
                    return Err(format!("The provided seed of \"{}\", must be divisible by the number of columns: {}, if the number of rows is not provided", seed.unwrap(), columns))
                }
            } else if rows_parameter.is_none() && columns_parameter.is_none() {
                if Self::is_perfect_square(seed_length) {
                    let sqrt = (seed_length as f32).sqrt() as i32;
                    columns = sqrt;
                    rows = sqrt;
                } else {
                    return Err(format!("The provided seed of \"{}\", must be of a square size (has an integer square root) if the number of rows and columns are not provided", seed.unwrap()))
                }
            } else if rows_parameter.is_some() && columns_parameter.is_some() {
                let simulation_area = rows * columns;
                if simulation_area != seed_length {
                    return Err(format!("Simulation area (rows * columns) of {}, must equal seed length of {}", simulation_area, seed_length))
                }
            }
        } else {
            if rows_parameter.is_some() && columns_parameter.is_some() {
                calculated_seed = seed.unwrap_or(Simulation::random_seed_string(rows, columns));
            } else if rows_parameter.is_some() && columns_parameter.is_none() {
                calculated_seed = seed.unwrap_or(Simulation::random_seed_string(rows, rows));
            } else if columns_parameter.is_some() && rows_parameter.is_none() {
                calculated_seed = seed.unwrap_or(Simulation::random_seed_string(columns, columns));
            } else if rows_parameter.is_none() && columns_parameter.is_none() {
                return Err("One of the following must be provided: rows, columns, or seed".to_string())
            }
        }
        Ok(Simulation {
            seed: calculated_seed.clone(),
            surface_type,
            rows,
            columns,
            generation: Simulation::seed_string_to_generation(calculated_seed.clone(), columns).unwrap(),
            generation_iteration: 0,
        })
    }

    pub(crate) fn new_spheroid(rows: i32, columns: i32, seed: String) -> Simulation { Self::new(Some(rows), Some(columns), Spheroid, Some(seed)).unwrap() }
    pub(crate) fn new_plane(rows: i32, columns: i32, seed: String) -> Simulation { Self::new(Some(rows), Some(columns), Plane, Some(seed)).unwrap() }
    pub(crate) fn new_horizontal_loop(rows: i32, columns: i32, seed: String) -> Simulation { Self::new(Some(rows), Some(columns), HorizontalLoop, Some(seed)).unwrap() }
    pub(crate) fn new_vertical_loop(rows: i32, columns: i32, seed: String) -> Simulation { Self::new(Some(rows), Some(columns), VerticalLoop, Some(seed)).unwrap() }

    pub(crate) fn new_spheroid_rand(rows: i32, columns: i32) -> Simulation { Self::new(Some(rows), Some(columns), Spheroid, None).unwrap() }
    pub(crate) fn new_plane_rand(rows: i32, columns: i32) -> Simulation { Self::new(Some(rows), Some(columns), Plane, None).unwrap() }
    pub(crate) fn new_horizontal_loop_rand(rows: i32, columns: i32) -> Simulation { Self::new(Some(rows), Some(columns), HorizontalLoop, None).unwrap() }
    pub(crate) fn new_vertical_loop_rand(rows: i32, columns: i32) -> Simulation { Self::new(Some(rows), Some(columns), VerticalLoop, None).unwrap() }
}