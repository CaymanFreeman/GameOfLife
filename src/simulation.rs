//! Modeling and live manipulation for Game of Life simulations.
//!
//! # Example
//! ```rust,no_run
//! use game_of_life::simulation::{Simulation, SurfaceType};
//! use game_of_life::simulation_builder::SimulationBuilder;
//!
//! let mut simulation: Simulation = SimulationBuilder::new()
//!     .rows(4) // 4 rows high
//!     .columns(9) // 9 columns wide
//!     .surface_type(SurfaceType::Rectangle) // Rectangle (non-wrapping) surface
//!     .print(false) // Declaring that the simulation should not print generations (automatically)
//!     .display(false) // Declaring that the simulation should not display the generations in a window
//!     .cell_size(50) // Cell size of 50x50 pixels
//!     .build() // Build into a simulation
//!     .unwrap();
//!
//! // Simulate an iteration and print the generation
//! simulation.simulate_generation();
//! println!("{}", simulation);
//!
//! // Simulate 15 iterations and print the generation
//! simulation.simulate_generations(15);
//! println!("{}", simulation);
//!
//! // Reset the simulation to 0 iterations with a new random seed
//! simulation.reset_to_rand()
//! ```

use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::iter::repeat;
use std::thread::sleep;
use std::time::Duration;

use crate::cell::CellState::{ALIVE, DEAD};
use crate::cell::{Cell, ALIVE_CHAR, DEAD_CHAR};

use rand::{thread_rng, Rng, ThreadRng};

use crate::simulation::SurfaceType::*;
use crate::simulation_window::SimulationWindowData;

/// Represents the surface type of a simulation (how wrapping will behave).
#[derive(Clone, Debug)]
pub enum SurfaceType {
    /// A spherical surface where cells wrap around on every edge.
    Ball,
    /// A cylindrical surface where cells wrap around horizontally (left/right).
    HorizontalLoop,
    /// A cylindrical surface where cells wrap around vertically (top/bottom).
    VerticalLoop,
    /// A rectangular surface with no wrapping.
    Rectangle,
}

/// Represents a simulation of the Game of Life.
pub struct Simulation {
    /// The initial seed string used to generate the simulation.
    pub seed: String,
    /// The surface type (affects wrapping) of the simulation.
    pub surface_type: SurfaceType,
    /// The number of rows in the simulation grid.
    pub rows: u16,
    /// The number of columns in the simulation grid.
    pub columns: u16,
    /// The current generation of cells in the simulation.
    pub generation: HashSet<Cell>,
    /// The current iteration or generation number of the simulation.
    pub generation_iteration: u128,
    /// A history of previous generations, used for rolling back the simulation.
    pub save_history: Vec<HashSet<Cell>>,
    /// The maximum number of generations to retain in the save history.
    pub maximum_saves: u128,
    /// A flag indicating whether the simulation should be displayed in a window.
    pub display: bool,
    /// A flag indicating whether the simulation should be printed to the console.
    pub print: bool,
    /// Data related to the display window for the simulation, if applicable.
    pub(crate) window_data: Option<SimulationWindowData>,
}

impl Clone for Simulation {
    /// Creates a deep clone of the `Simulation` instance.
    fn clone(&self) -> Self {
        Simulation {
            seed: self.seed.clone(),
            surface_type: self.surface_type.clone(),
            rows: self.rows,
            columns: self.columns,
            generation: self.generation.clone(),
            generation_iteration: self.generation_iteration,
            save_history: self.save_history.clone(),
            maximum_saves: self.maximum_saves,
            display: self.display,
            print: self.print,
            window_data: self.window_data.clone(),
        }
    }
}

impl Display for Simulation {
    /// Renders the string representation of the current generation.
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if self.generation_iteration == 0 {
            write!(f, "SEED\n")?;
        } else {
            write!(f, "{}\n", self.generation_iteration)?;
        }
        for row in 0..self.rows {
            for column in 0..self.columns {
                write!(f, "{}", self.get_cell(row, column).as_char())?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Simulation {
    /// Returns the cell at the given row and column.
    fn get_cell(&self, row: u16, column: u16) -> Cell {
        let mut cell: Cell = Cell::new(ALIVE, row, column);
        if !self.generation.contains(&cell) {
            cell.state = DEAD;
        }
        return cell;
    }

    /// Counts the number of alive neighbor cells for the given cell.
    /// "If it ain't broke"
    fn get_alive_neighbors(&self, cell: Cell) -> u8 {
        let origin_row: u16 = cell.row;
        let origin_column: u16 = cell.column;
        let mut wrapping_vertically: bool = false;
        let mut wrapping_horizontally: bool = false;
        let mut bounded_vertically: bool = false;
        let mut bounded_horizontally: bool = false;
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

        let on_top_edge: bool = origin_row == 0;
        let on_bottom_edge: bool = origin_row == self.rows.clone() - 1;
        let on_left_edge: bool = origin_column == 0;
        let on_right_edge: bool = origin_column == self.columns.clone() - 1;

        let top_left_is_alive: bool = {
            let result: bool = (|| {
                if on_top_edge && bounded_vertically {
                    return false;
                }
                if on_left_edge && bounded_horizontally {
                    return false;
                }
                let neighbor_row: u16;
                let neighbor_column: u16;
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
        let top_center_is_alive: bool = {
            let result: bool = (|| {
                if on_top_edge && bounded_vertically {
                    return false;
                }
                let neighbor_row: u16;
                if on_top_edge && wrapping_vertically {
                    neighbor_row = self.rows.clone() - 1
                } else {
                    neighbor_row = origin_row.clone() - 1
                }
                self.get_cell(neighbor_row, origin_column.clone())
                    .is_alive()
            })();
            result
        };
        let top_right_is_alive: bool = {
            let result: bool = (|| {
                if on_top_edge && bounded_vertically {
                    return false;
                }
                if on_right_edge && bounded_horizontally {
                    return false;
                }
                let neighbor_row: u16;
                let neighbor_column: u16;
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
        let middle_left_is_alive: bool = {
            let result: bool = (|| {
                if on_left_edge && bounded_horizontally {
                    return false;
                }
                let neighbor_column: u16;
                if on_left_edge && wrapping_horizontally {
                    neighbor_column = self.columns.clone() - 1
                } else {
                    neighbor_column = origin_column.clone() - 1
                }
                self.get_cell(origin_row.clone(), neighbor_column)
                    .is_alive()
            })();
            result
        };
        let middle_right_is_alive: bool = {
            let result: bool = (|| {
                if on_right_edge && bounded_horizontally {
                    return false;
                }
                let neighbor_column: u16;
                if on_right_edge && wrapping_horizontally {
                    neighbor_column = 0;
                } else {
                    neighbor_column = origin_column.clone() + 1
                }
                self.get_cell(origin_row.clone(), neighbor_column)
                    .is_alive()
            })();
            result
        };
        let bottom_left_is_alive: bool = {
            let result: bool = (|| {
                if on_left_edge && bounded_horizontally {
                    return false;
                }
                if on_bottom_edge && bounded_vertically {
                    return false;
                }
                let neighbor_row: u16;
                let neighbor_column: u16;
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
        let bottom_center_is_alive: bool = {
            let result: bool = (|| {
                if on_bottom_edge && bounded_vertically {
                    return false;
                }
                let neighbor_row: u16;
                if on_bottom_edge && wrapping_vertically {
                    neighbor_row = 0;
                } else {
                    neighbor_row = origin_row.clone() + 1
                }
                self.get_cell(neighbor_row, origin_column.clone())
                    .is_alive()
            })();
            result
        };
        let bottom_right_is_alive: bool = {
            let result: bool = (|| {
                if on_bottom_edge && bounded_vertically {
                    return false;
                }
                if on_right_edge && bounded_horizontally {
                    return false;
                }
                let neighbor_row: u16;
                let neighbor_column: u16;
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

        let mut count: u8 = 0;
        if top_left_is_alive {
            count += 1
        }
        if top_center_is_alive {
            count += 1
        }
        if top_right_is_alive {
            count += 1
        }
        if middle_left_is_alive {
            count += 1
        }
        if middle_right_is_alive {
            count += 1
        }
        if bottom_left_is_alive {
            count += 1
        }
        if bottom_center_is_alive {
            count += 1
        }
        if bottom_right_is_alive {
            count += 1
        }
        count
    }

    /// Saves the current generation to the save history.
    fn save_generation(&mut self) {
        if self.save_history.len() == self.maximum_saves as usize {
            self.save_history.remove(0);
        }
        self.save_history.push(self.generation.clone());
    }

    /// Rolls back the simulation by the specified number of generations.
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
        if self.window_data.is_some() {
            self.draw_generation()
        }
    }

    /// Rolls back the simulation by one generation.
    pub fn rollback_generation(&mut self) {
        self.rollback_generations(1)
    }

    /// Simulates the specified number of generations.
    pub fn simulate_generations(&mut self, iterations: u128) {
        if iterations == 0 {
            return;
        }
        self.save_generation();
        for _ in 0..iterations {
            let mut new_generation: HashSet<Cell> = self.generation.clone();
            let mut row: u16 = 0;
            while row < self.rows {
                let mut column: u16 = 0;
                while column < self.columns {
                    let mut cell: Cell = self.get_cell(row.clone(), column.clone());
                    let alive_neighbors: u8 = self.get_alive_neighbors(cell.clone());
                    let cell_alive: bool = cell.is_alive();
                    if cell_alive {
                        if alive_neighbors < 2 || alive_neighbors > 3 {
                            new_generation.remove(&cell);
                        }
                    } else {
                        if alive_neighbors == 3 {
                            cell.state = ALIVE;
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
        if self.display {
            self.draw_generation()
        }
        if self.print {
            println!("{}", self)
        }
    }

    /// Simulates one generation.
    pub fn simulate_generation(&mut self) {
        self.simulate_generations(1)
    }

    /// Simulates generations continuously with a specified cooldown period.
    pub fn simulate_continuous_generations(
        &mut self,
        cooldown: Duration,
        stop_when_finished: bool,
    ) {
        loop {
            self.simulate_generation();
            if stop_when_finished && self.is_finished() {
                break;
            }
            sleep(cooldown)
        }
    }

    /// Returns the count of alive cells in the current generation.
    pub fn alive_count(&self) -> u64 {
        self.generation.len() as u64
    }

    /// Returns the proportion of alive cells in the current generation.
    pub fn alive_proportion(&self) -> f64 {
        self.alive_count() as f64 / self.area() as f64
    }

    /// Returns the total area (number of cells) in the simulation.
    pub fn area(&self) -> u16 {
        self.rows * self.columns
    }

    /// Resets the simulation to the initial seed.
    pub fn reset(&mut self) {
        let seed: String = self.seed.clone();
        self.generation = generation_from_string(String::from(seed), self.columns).unwrap();
        self.generation_iteration = 0;
    }

    /// Resets the simulation to the specified seed.
    pub fn reset_to(&mut self, seed: &str) {
        self.generation = generation_from_string(String::from(seed), self.columns).unwrap();
        self.generation_iteration = 0;
    }

    /// Resets the simulation to a random seed.
    pub fn reset_to_rand(&mut self) {
        let seed: String = random_seed(self.rows, self.columns);
        self.generation = generation_from_string(String::from(seed), self.columns).unwrap();
        self.generation_iteration = 0;
    }

    /// Returns true if the simulation is in a still state (a period of 1).
    pub fn is_still(&self) -> bool {
        self.is_periodic(1)
    }

    /// Returns true if the simulation is in a periodic state with the specified period.
    pub fn is_periodic(&self, period: usize) -> bool {
        self.save_history.len() >= period
            && self.generation == self.save_history[self.save_history.len() - (period)]
    }

    /// Returns true if the simulation has reached a finished state (has any periodic state).
    pub fn is_finished(&self) -> bool {
        self.save_history.contains(&self.generation)
    }

    /// Returns the string representation of the current generation.
    pub fn generation_string(&self) -> String {
        string_from_generation(self.generation.clone(), self.rows, self.columns)
    }
}

/// Converts a string seed into a `HashSet` of `Cell` instances.
pub fn generation_from_string(seed: String, columns: u16) -> Result<HashSet<Cell>, String> {
    let mut generation: HashSet<Cell> = HashSet::new();
    let values: Vec<char> = seed.chars().collect();
    for i in 0..values.len() {
        let index: u16 = i as u16;
        let row_index: u16 = index.clone() / columns.clone();
        let column_index: u16 = index % columns.clone();
        let value: char = values.get(i).unwrap().clone();
        match value {
            ALIVE_CHAR => {
                generation.insert(Cell::new(ALIVE, row_index, column_index));
            }
            DEAD_CHAR => {}
            _ => {
                return Err(format!(
                    "Unexpected seed character of \'{}\', seeds must only contain \'{}\' or \'{}\'",
                    value, DEAD_CHAR, ALIVE_CHAR
                ));
            }
        };
    }
    Ok(generation)
}

/// Converts a `HashSet` of `Cell` instances into a `String` representation.
pub fn string_from_generation(generation: HashSet<Cell>, rows: u16, columns: u16) -> String {
    let mut generation_characters: Vec<char> =
        repeat(DEAD_CHAR).take((rows * columns) as usize).collect();
    for cell in generation {
        generation_characters[(cell.row * columns + cell.column) as usize] = ALIVE_CHAR;
    }
    generation_characters.iter().collect()
}

/// Generates a random seed `String` for the specified number of rows and columns.
pub fn random_seed(rows: u16, columns: u16) -> String {
    let length: usize = (rows * columns).into();
    let mut rng: ThreadRng = thread_rng();
    (0..length)
        .map(|_| if rng.gen() { ALIVE_CHAR } else { DEAD_CHAR })
        .collect()
}
