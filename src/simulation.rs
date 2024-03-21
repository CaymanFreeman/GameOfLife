use std::collections::HashSet;
use std::hash::Hash;
use num_traits::{FromPrimitive, ToPrimitive, Unsigned};
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

pub struct Simulation<U: Unsigned + Eq + Hash> {
    pub seed: String,
    pub surface_type: SurfaceType,
    pub rows: U,
    pub columns: U,
    pub generation: HashSet<Cell<U>>,
    pub generation_iteration: u128,
}

impl<U: Unsigned + Clone + ToPrimitive + FromPrimitive + Eq + Hash + PartialOrd> Simulation<U> {

    pub fn get_seed_string(&self) -> String { self.seed.clone() }

    pub fn get_generation_string(&self) -> String {
        let mut current_generation = String::new();
        let mut row = U::zero();
        while row < self.rows {
            let mut column = U::zero();
            while column < self.columns {
                current_generation.push(self.get_cell(row.clone(), column.clone()).to_seed_value());
                column = column + U::one();
            }
            row = row + U::one();
        }
        current_generation
    }

    pub fn print_seed_generation(&self, print_with_grid: Option<bool>) {
        let print_with_grid = print_with_grid.unwrap_or(false);
        println!("SEED: {}", self.seed);
        if print_with_grid {
            let first_iteration = Self::from_seed_generation(self);
            let mut row = U::zero();
            while row < first_iteration.rows {
                let mut column = U::zero();
                while column < first_iteration.columns {
                    print!("{}", first_iteration.get_cell(row.clone(), column.clone()).to_display());
                    column = column + U::one();
                }
                print!("\n");
                row = row + U::one();
            }
        }
    }

    pub fn print_current_generation(&self) {
        if self.generation_iteration == 0 {
            println!("SEED GENERATION");
        } else {
            println!("{}", self.generation_iteration);
        }
        let mut row = U::zero();
        while row < self.rows {
            let mut column = U::zero();
            while column < self.columns {
                print!("{}", self.get_cell(row.clone(), column.clone()).to_display());
                column = column + U::one();
            }
            print!("\n");
            row = row + U::one();
        }
    }

    fn get_cell(&self, row: U, column: U) -> Cell<U> {
        let mut cell = Cell::new_alive(row, column);
        if !self.generation.contains(&cell) {
            cell.state = Dead;
        }
        return cell
    }

    fn get_alive_neighbors(&self, cell: Cell<U>) -> u8 {
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

        let on_top_edge = origin_row == U::zero();
        let on_bottom_edge = origin_row == self.rows.clone() - U::one();
        let on_left_edge = origin_column == U::zero();
        let on_right_edge = origin_column == self.columns.clone() - U::one();

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
                    neighbor_row = self.rows.clone() - U::one()
                } else {
                    neighbor_row = origin_row.clone() - U::one()
                }
                if on_left_edge && wrapping_horizontally {
                    neighbor_column = self.columns.clone() - U::one()
                } else {
                    neighbor_column = origin_column.clone() - U::one()
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
                    neighbor_row = self.rows.clone() - U::one()
                } else {
                    neighbor_row = origin_row.clone() - U::one()
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
                    neighbor_row = self.rows.clone() - U::one()
                } else {
                    neighbor_row = origin_row.clone() - U::one()
                }
                if on_right_edge && wrapping_horizontally {
                    neighbor_column = U::zero();
                } else {
                    neighbor_column = origin_column.clone() + U::one()
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
                    neighbor_column = self.columns.clone() - U::one()
                } else {
                    neighbor_column = origin_column.clone() - U::one()
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
                    neighbor_column = U::zero();
                } else {
                    neighbor_column = origin_column.clone() + U::one()
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
                    neighbor_row = U::zero();
                } else {
                    neighbor_row = origin_row.clone() + U::one()
                }
                if on_left_edge && wrapping_horizontally {
                    neighbor_column = self.columns.clone() - U::one()
                } else {
                    neighbor_column = origin_column.clone() - U::one()
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
                    neighbor_row = U::zero();
                } else {
                    neighbor_row = origin_row.clone() + U::one()
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
                    neighbor_row = U::zero();
                } else {
                    neighbor_row = origin_row.clone() + U::one()
                }
                if on_right_edge && wrapping_horizontally {
                    neighbor_column = U::zero();
                } else {
                    neighbor_column = origin_column.clone() + U::one()
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

    pub fn simulate_generations(&mut self, iterations: u128) {
        if iterations < 1 {
            return
        }
        for _ in 0..iterations {
            let mut new_generation = self.generation.clone();
            let mut row = U::zero();
            while row < self.rows {
                let mut column = U::zero();
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
                    column = column + U::one();
                }
                row = row + U::one();
            }
            self.generation = new_generation;
            self.generation_iteration += 1;
        }
    }

    pub fn simulate_generation(&mut self) {
        self.simulate_generations(1)
    }

    fn is_perfect_square(number: U) -> bool {
        let sqrt = U::from_f32(U::to_f32(&number).unwrap().sqrt()).unwrap();
        U::to_u128(&sqrt).unwrap().pow(2) == U::to_u128(&number.clone()).unwrap()
    }

    pub fn clone(&self) -> Simulation<U> {
        Simulation {
            seed: self.seed.clone(),
            surface_type: self.surface_type.clone(),
            rows: self.rows.clone(),
            columns: self.columns.clone(),
            generation: self.generation.clone(),
            generation_iteration: self.generation_iteration,
        }
    }

    pub fn from_seed_generation(simulation: &Simulation<U>) -> Simulation<U> {
        Simulation {
            seed: simulation.seed.clone(),
            surface_type: simulation.surface_type.clone(),
            rows: simulation.rows.clone(),
            columns: simulation.columns.clone(),
            generation: seed_string_to_generation(simulation.seed.clone(), simulation.columns.clone()).unwrap(),
            generation_iteration: 0,
        }
    }

    pub fn new(rows_parameter: Option<U>, columns_parameter: Option<U>, surface_type: SurfaceType, seed: Option<String>) -> Result<Simulation<U>, String> {
        let mut calculated_seed = String::new();
        let mut rows = rows_parameter.clone().unwrap_or(U::zero());
        let mut columns = columns_parameter.clone().unwrap_or(U::zero());
        if seed.is_some() {
            calculated_seed = seed.clone().unwrap();
            let seed_length = U::from_usize(seed.clone().unwrap().len()).unwrap();
            if rows_parameter.is_some() && columns_parameter.is_none() {
                if seed_length.clone() % rows.clone() == U::zero() {
                    columns = seed_length.clone() / rows.clone();
                } else {
                    return Err(format!("The provided seed of \"{}\", must be divisible by the number of rows: {}, \
                    if the number of columns is not provided", seed.unwrap(), U::to_u128(&rows.clone()).unwrap()))
                }
            } else if columns_parameter.is_some() && rows_parameter.is_none() {
                if seed_length.clone() % columns.clone() == U::zero() {
                    rows = seed_length.clone() / columns.clone();
                } else {
                    return Err(format!("The provided seed of \"{}\", must be divisible by the number of columns: {}, \
                    if the number of rows is not provided", seed.unwrap(), U::to_u128(&columns.clone()).unwrap()))
                }
            } else if rows_parameter.is_none() && columns_parameter.is_none() {
                if Self::is_perfect_square(seed_length.clone()) {
                    let sqrt = U::from_f32(U::to_f32(&seed_length).unwrap().sqrt()).unwrap();
                    columns = sqrt.clone();
                    rows = sqrt.clone();
                } else {
                    return Err(format!("The provided seed of \"{}\", must be of a square size (has an integer square root) \
                    if the number of rows and columns are not provided", seed.unwrap()))
                }
            } else if rows_parameter.is_some() && columns_parameter.is_some() {
                let simulation_area = rows.clone() * columns.clone();
                if simulation_area != seed_length.clone() {
                    return Err(format!("Simulation area (rows * columns) of {}, must equal seed length of {}", U::to_u128(&simulation_area).unwrap(), U::to_u128(&seed_length).unwrap()))
                }
            }
        } else {
            if rows_parameter.is_some() && columns_parameter.is_some() {
                calculated_seed = seed.unwrap_or(random_seed_string(rows.clone(), columns.clone()));
            } else if rows_parameter.is_some() && columns_parameter.is_none() {
                calculated_seed = seed.unwrap_or(random_seed_string(rows.clone(), rows.clone()));
            } else if columns_parameter.is_some() && rows_parameter.is_none() {
                calculated_seed = seed.unwrap_or(random_seed_string(columns.clone(), columns.clone()));
            } else if rows_parameter.is_none() && columns_parameter.is_none() {
                return Err("One of the following must be provided: rows, columns, or seed".to_string())
            }
        }
        Ok(Simulation {
            seed: calculated_seed.clone(),
            surface_type,
            generation: seed_string_to_generation(calculated_seed.clone(), columns.clone()).unwrap(),
            rows,
            columns,
            generation_iteration: 0,
        })
    }

    pub fn new_ball(rows: U, columns: U, seed: String) -> Simulation<U> { Self::new(Some(rows), Some(columns), Ball, Some(seed)).unwrap() }
    pub fn new_rectangle(rows: U, columns: U, seed: String) -> Simulation<U> { Self::new(Some(rows), Some(columns), Rectangle, Some(seed)).unwrap() }
    pub fn new_square(size: U, seed: String) -> Simulation<U> { Self::new(Some(size), None, Rectangle, Some(seed)).unwrap() }
    pub fn new_horizontal_loop(rows: U, columns: U, seed: String) -> Simulation<U> { Self::new(Some(rows), Some(columns), HorizontalLoop, Some(seed)).unwrap() }
    pub fn new_vertical_loop(rows: U, columns: U, seed: String) -> Simulation<U> { Self::new(Some(rows), Some(columns), VerticalLoop, Some(seed)).unwrap() }

    pub fn new_ball_rand(rows: U, columns: U) -> Simulation<U> { Self::new(Some(rows), Some(columns), Ball, None).unwrap() }
    pub fn new_rectangle_rand(rows: U, columns: U) -> Simulation<U> { Self::new(Some(rows), Some(columns), Rectangle, None).unwrap() }
    pub fn new_square_rand(size: U) -> Simulation<U> { Self::new(Some(size), None, Rectangle, None).unwrap() }
    pub fn new_horizontal_loop_rand(rows: U, columns: U) -> Simulation<U> { Self::new(Some(rows), Some(columns), HorizontalLoop, None).unwrap() }
    pub fn new_vertical_loop_rand(rows: U, columns: U) -> Simulation<U> { Self::new(Some(rows), Some(columns), VerticalLoop, None).unwrap() }
}