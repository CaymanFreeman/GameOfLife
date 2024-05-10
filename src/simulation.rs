//! Modeling and live manipulation for Game of Life simulations.
//!
//! # Example
//! ```rust,no_run
//! use simple_game_of_life::simulation::{Simulation, SurfaceType};
//! use simple_game_of_life::simulation_builder::SimulationBuilder;
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

use crate::rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::prelude::ThreadRng;
use rand::thread_rng;

use crate::cell::CellState::{ALIVE, DEAD};
use crate::cell::{Cell, ALIVE_CHAR, DEAD_CHAR};
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
    ///
    /// # Description
    /// This function is part of the `Display` trait implementation for the `Simulation` struct.
    /// It is responsible for generating a textual representation of the current generation,
    /// which can be used for printing or displaying the simulation state.
    ///
    /// This function writes the following information to the provided `Formatter`:
    ///
    /// 1. If the current iteration is 0, it writes the string "SEED".
    /// 2. Otherwise, it writes the current iteration number.
    /// 3. For each row in the simulation grid, it iterates through the columns and writes the
    /// corresponding character representation (either `'*'` for alive cells or `'-'` for
    /// dead cells) obtained by calling the `as_char` method of the `Cell` struct.
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
    ///
    /// # Description
    /// This function retrieves the `Cell` instance representing the cell at the specified
    /// row and column coordinates in the simulation grid.
    ///
    /// It first creates a new `Cell` instance with the `ALIVE` state and the provided
    /// row and column indices.
    ///
    /// Then, it checks if this `Cell` exists in the current generation (`self.generation`).
    /// If the `Cell` is not found in the generation, its state is set to `DEAD`.
    ///
    /// # Arguments
    /// * `row` - The row index of the cell to retrieve.
    /// * `column` - The column index of the cell to retrieve.
    ///
    /// # Returns
    /// A `Cell` instance representing the cell at the specified row and column coordinates
    /// in the simulation grid, with its state set to `ALIVE` if it exists in the current
    /// generation, or `DEAD` otherwise.
    fn get_cell(&self, row: u16, column: u16) -> Cell {
        let mut cell: Cell = Cell::new(ALIVE, row, column);
        if !self.generation.contains(&cell) {
            cell.state = DEAD;
        }
        return cell;
    }

    /// Counts the number of alive neighbor cells for the given cell.
    ///
    /// # Description
    /// This function determines the number of alive neighbor cells surrounding the specified
    /// `Cell` instance in the current generation of the simulation.
    ///
    /// It considers all eight neighboring cells (top, bottom, left, right, and four diagonals)
    /// and counts how many of them are alive.
    ///
    /// This function takes into account the surface type of the simulation to handle wrapping
    /// behavior correctly.
    ///
    /// To maintain the use of unsigned integers, this function is built to never
    /// hold or calculate a negative number.
    ///
    /// If the simulation has a wrapping surface type (e.g., `Ball`, `HorizontalLoop`,
    /// `VerticalLoop`), this function adjusts the neighbor cell coordinates accordingly
    /// to wrap around the edges of the grid.
    ///
    /// # Arguments
    /// * `cell` - The `Cell` instance for which to count the alive neighbors.
    ///
    /// # Returns
    /// An `u8` value representing the number of alive neighbor cells surrounding the specified
    /// `Cell` instance.
    ///
    /// #
    /// I don't remember how I came up with this function, but it works, and it haunts me.
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
    ///
    /// # Description
    /// This function adds a copy of the current generation to the simulation's save history.
    /// The save history is a vector that stores previous generations, allowing the simulation
    /// to be rolled back to a previous state if needed.
    ///
    /// This function maintains a maximum number of saved generations specified by the
    /// `maximum_saves` field.
    ///
    /// When the save history reaches the maximum size, the oldest generation is removed before
    /// adding the new generation to the end of the vector.
    ///
    /// Saving generations is essential for enabling features like rolling back the simulation
    /// or detecting periodic or still states, where the current generation matches a previous
    /// generation in the save history.
    fn save_generation(&mut self) {
        if self.save_history.len() == self.maximum_saves as usize {
            self.save_history.remove(0);
        }
        self.save_history.push(self.generation.clone());
    }

    /// Rolls back the simulation by the specified number of generations.
    ///
    /// # Description
    /// This function allows you to undo a certain number of iterations in the simulation by
    /// restoring the state of the simulation to a previous generation stored in the save history.
    ///
    /// If the requested number of rollback iterations exceeds the available save history,
    /// the simulation will be rolled back to the earliest saved generation.
    ///
    /// After rolling back the specified number of generations, if the simulation is set to
    /// display in a window, the current generation is drawn on the display window.
    ///
    /// # Arguments
    /// * `iterations` - The number of generations to roll back.
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
        if self.display {
            self.draw_generation()
        }
    }

    /// Rolls back one generation.
    pub fn rollback_generation(&mut self) {
        self.rollback_generations(1)
    }

    /// Simulates the specified number of generations in the simulation.
    ///
    /// # Description
    /// This function advances the simulation by the given number of iterations, updating the
    /// current generation based on the rules of the Game of Life.
    ///
    /// For each iteration, the following steps are performed:
    ///
    /// 1. Save the current generation to the save history.
    /// 2. Create a new `HashSet` to store the next generation.
    /// 3. Iterate through each cell in the current generation.
    ///
    ///    a. Count the number of alive neighbors for the current cell.
    ///
    ///    b. If the cell is alive and has fewer than 2 or more than 3 alive neighbors, mark it
    /// as dead in the next generation.
    ///
    ///    c. If the cell is dead and has exactly 3 alive neighbors, mark it as alive in the
    /// next generation.
    ///
    /// 4. Update the current generation to the new generation.
    ///
    /// 5. Increment the generation iteration counter.
    ///
    /// After simulating the specified number of iterations, if the simulation is set to display
    /// in a window, the current generation is drawn on the display window.
    ///
    /// If the simulation is set to print to the console, the current generation is printed to
    /// the console.
    ///
    /// # Arguments
    /// * `iterations` - The number of generations to simulate.
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
    /// # Note
    /// Resetting is preferred over creating a new simulation since it will continue in the same
    /// window. You can not have multiple windows at once.
    pub fn reset(&mut self) {
        let seed: String = self.seed.clone();
        self.generation = generation_from_string(String::from(seed), self.columns).unwrap();
        self.generation_iteration = 0;
    }

    /// Resets the simulation to the specified seed.
    /// # Note
    /// Resetting is preferred over creating a new simulation since it will continue in the same
    /// window. You can not have multiple windows at once.
    pub fn reset_to(&mut self, seed: &str) {
        self.generation = generation_from_string(String::from(seed), self.columns).unwrap();
        self.seed = String::from(seed);
        self.generation_iteration = 0;
    }

    /// Resets the simulation to a random seed.
    ///
    /// # Note
    /// Resetting is preferred over creating a new simulation since it will continue in the same
    /// window. You can not have multiple windows at once.
    pub fn reset_to_rand(&mut self) {
        let seed: String = random_seed(self.rows, self.columns);
        self.generation = generation_from_string(String::from(seed.clone()), self.columns).unwrap();
        self.seed = seed;
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
///
/// # Description
/// This function takes a string seed representation of a generation and converts it into a
/// `HashSet` of `Cell` instances. The string seed should consist of the characters `'*'`
/// (alive) and `'-'` (dead), representing the state of each cell in the generation.
///
/// This function iterates through each character in the seed string and creates a `Cell`
/// instance for each alive cell (`'*'`), with the appropriate row and column indices based on
/// the position of the character in the string and the provided number of columns.
///
/// If the seed string contains any characters other than `'*'` or `'-'`, an error is returned.
///
/// The resulting `HashSet` of `Cell` instances represents the generation specified by the seed
/// string.
///
/// # Arguments
/// * `seed` - A string representation of the generation, where `'*'` represents an alive cell
/// and `'-'` represents a dead cell.
/// * `columns` - The number of columns in the generation grid, used to determine the row and
/// column indices of each cell from its position in the seed string.
///
/// # Returns
/// * `Ok(HashSet<Cell>)` - A `HashSet` containing `Cell` instances representing the alive cells
/// in the generation specified by the seed string.
/// * `Err(String)` - An error message if the seed string contains invalid characters.
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
///
/// # Description
/// This function takes a `HashSet` of `Cell` instances representing a generation and converts
/// it into a string representation. The resulting string consists of the characters `'*'`
/// (alive) and `'-'` (dead), representing the state of each cell in the generation.
///
/// This function iterates through each row and column of the generation grid and appends the
/// corresponding character (`'*'` or `'-'`) to the output string based on whether a `Cell`
/// instance exists in the provided `HashSet` for that row and column.
///
/// The resulting string is a compact representation of the generation, and can be used for
/// storage or display purposes.
///
/// # Arguments
/// * `generation` - A `HashSet` of `Cell` instances representing the alive cells in the
/// generation.
/// * `rows` - The number of rows in the generation grid.
/// * `columns` - The number of columns in the generation grid.
///
/// # Returns
/// A `String` representation of the generation, where `'*'` represents an alive cell and `'-'`
/// represents a dead cell.
pub fn string_from_generation(generation: HashSet<Cell>, rows: u16, columns: u16) -> String {
    let mut generation_characters: Vec<char> =
        repeat(DEAD_CHAR).take((rows * columns) as usize).collect();
    for cell in generation {
        generation_characters[(cell.row * columns + cell.column) as usize] = ALIVE_CHAR;
    }
    generation_characters.iter().collect()
}

/// Generates a random seed `String` for the specified number of rows and columns with a random alive probability.
///
/// # Description
/// This function creates a random seed string representing a generation with the given number
/// of rows and columns and a randomly determined probability for a cell to be alive.
///
/// The seed string consists of the characters `'*'` (alive) and `'-'` (dead), with the probability
/// of `'*'` being randomly determined for each call.
///
/// The resulting seed string can be used as input for the `generation_from_string` function to
/// create a randomly initialized generation.
///
/// # Arguments
/// * `rows` - The number of rows in the generation grid.
/// * `columns` - The number of columns in the generation grid.
///
/// # Returns
/// A `String` representation of a randomly generated generation, where `'*'` represents an alive
/// cell and `'-'` represents a dead cell.
pub fn random_seed(rows: u16, columns: u16) -> String {
    let length: usize = (rows * columns).into();
    let mut rng: ThreadRng = thread_rng();
    let dist = Uniform::from(0.0..1.0);
    let alive_probability = dist.sample(&mut rng);
    (0..length)
        .map(|_| {
            if dist.sample(&mut rng) < alive_probability {
                ALIVE_CHAR
            } else {
                DEAD_CHAR
            }
        })
        .collect()
}

/// Generates a random seed `String` for the specified number of rows and columns with a given alive probability.
///
/// # Description
/// This function creates a random seed string representing a generation with the given number
/// of rows and columns and a specified probability for a cell to be alive.
///
/// The seed string consists of the characters `'*'` (alive) and `'-'` (dead), with the probability
/// of `'*'` being determined by the `alive_probability` parameter.
///
/// The resulting seed string can be used as input for the `generation_from_string` function to
/// create a randomly initialized generation.
///
/// # Arguments
/// * `rows` - The number of rows in the generation grid.
/// * `columns` - The number of columns in the generation grid.
/// * `alive_probability` - The probability of a cell being alive.
///
/// # Returns
/// A `String` representation of a randomly generated generation, where `'*'` represents an alive
/// cell and `'-'` represents a dead cell.
pub fn random_seed_probability(rows: u16, columns: u16, alive_probability: f64) -> String {
    let length: usize = (rows * columns).into();
    let mut rng: ThreadRng = thread_rng();
    let dist = Uniform::from(0.0..1.0);
    (0..length)
        .map(|_| {
            if dist.sample(&mut rng) < alive_probability {
                ALIVE_CHAR
            } else {
                DEAD_CHAR
            }
        })
        .collect()
}
