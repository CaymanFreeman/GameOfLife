//! The system for creating and customizing `Simulation`s.
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
//!     .display(true) // Declaring that the simulation should display the generations in a window
//!     .cell_size(50) // Cell size of 50x50 pixels
//!     .build() // Build into a simulation
//!     .unwrap();
//! ```

use crate::simulation::SurfaceType::Rectangle;
use crate::simulation::{generation_from_string, random_seed, Simulation, SurfaceType};
use crate::simulation_window::SimulationWindowData;
use simple::Window;

/// A builder for configuring and creating a new `Simulation`.
pub struct SimulationBuilder {
    /// The number of rows in the simulation grid.
    rows: Option<u16>,
    /// The number of columns in the simulation grid.
    columns: Option<u16>,
    /// The surface type (affects wrapping) of the simulation.
    surface_type: SurfaceType,
    /// The initial seed string used to generate the simulation.
    seed: Option<String>,
    /// The maximum number of generations to retain in the save history.
    maximum_saves: u128,
    /// The width of each cell in the display in pixels.
    cell_width: Option<u16>,
    /// The height of each cell in the display in pixels.
    cell_height: Option<u16>,
    /// The red component of the cell color in the display.
    cell_color_red: u8,
    /// The green component of the cell color in the display.
    cell_color_green: u8,
    /// The blue component of the cell color in the display.
    cell_color_blue: u8,
    /// The alpha (transparency) component of the cell color in the display.
    cell_color_alpha: u8,
    /// The red component of the background color in the display.
    background_color_red: u8,
    /// The green component of the background color in the display.
    background_color_green: u8,
    /// The blue component of the background color in the display.
    background_color_blue: u8,
    /// The alpha (transparency) component of the background color in the display.
    background_color_alpha: u8,
    /// The red component of the grid line color in the display.
    line_color_red: u8,
    /// The green component of the grid line color in the display.
    line_color_green: u8,
    /// The blue component of the grid line color in the display.
    line_color_blue: u8,
    /// The alpha (transparency) component of the grid line color in the display.
    line_color_alpha: u8,
    /// The thickness of the grid lines in the display.
    line_thickness: u16,
    /// The width of the display window in pixels.
    window_width: Option<u16>,
    /// The height of the display window in pixels.
    window_height: Option<u16>,
    /// The title of the display window.
    window_title: String,
    /// A flag indicating whether the simulation should be displayed in a window.
    display: bool,
    /// A flag indicating whether the simulation should be printed to the console.
    print: bool,
}

impl Default for SimulationBuilder {
    /// Defines the default configuration settings for a `SimulationBuilder`.
    fn default() -> Self {
        Self {
            rows: None,
            columns: None,
            surface_type: Rectangle,
            seed: None,
            maximum_saves: 100,
            cell_width: None,
            cell_height: None,
            cell_color_red: 255,
            cell_color_green: 255,
            cell_color_blue: 0,
            cell_color_alpha: 255,
            background_color_red: 255,
            background_color_green: 255,
            background_color_blue: 255,
            background_color_alpha: 255,
            line_color_red: 0,
            line_color_green: 0,
            line_color_blue: 0,
            line_color_alpha: 255,
            line_thickness: 5,
            window_width: None,
            window_height: None,
            window_title: String::from("Game of Life"),
            display: false,
            print: false,
        }
    }
}

impl SimulationBuilder {
    /// Creates a new `SimulationBuilder` instance with default configuration settings.
    pub fn new() -> Self {
        Default::default()
    }

    /// Enables or disables printing the simulation to the console.
    pub fn print(mut self, print: bool) -> Self {
        self.print = print;
        self
    }

    /// Enables or disables displaying the simulation in a window.
    pub fn display(mut self, display: bool) -> Self {
        self.display = display;
        self
    }

    /// Sets the width of the display window.
    pub fn window_width(mut self, window_width: u16) -> Self {
        self.window_width = Some(window_width);
        self
    }

    /// Sets the height of the display window.
    pub fn window_height(mut self, window_height: u16) -> Self {
        self.window_height = Some(window_height);
        self
    }

    /// Sets the width and height of the display window to the same value.
    pub fn window_size(mut self, window_size: u16) -> Self {
        self.window_width = Some(window_size);
        self.window_height = Some(window_size);
        self
    }

    /// Sets the title of the display window.
    pub fn window_title(mut self, window_title: &str) -> Self {
        self.window_title = String::from(window_title);
        self
    }

    /// Sets the width of each cell in the display.
    pub fn cell_width(mut self, cell_width: u16) -> Self {
        self.cell_width = Some(cell_width);
        self
    }

    /// Sets the height of each cell in the display.
    pub fn cell_height(mut self, cell_height: u16) -> Self {
        self.cell_height = Some(cell_height);
        self
    }

    /// Sets the width and height of each cell in the display to the same value.
    pub fn cell_size(mut self, cell_size: u16) -> Self {
        self.cell_width = Some(cell_size);
        self.cell_height = Some(cell_size);
        self
    }

    /// Sets the color of the cells in the display.
    pub fn cell_color(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        self.cell_color_red = red;
        self.cell_color_green = green;
        self.cell_color_blue = blue;
        self.cell_color_alpha = alpha;
        self
    }

    /// Sets the red component of the cell color in the display.
    pub fn cell_color_red(mut self, red: u8) -> Self {
        self.cell_color_red = red;
        self
    }

    /// Sets the green component of the cell color in the display.
    pub fn cell_color_green(mut self, green: u8) -> Self {
        self.cell_color_green = green;
        self
    }

    /// Sets the blue component of the cell color in the display.
    pub fn cell_color_blue(mut self, blue: u8) -> Self {
        self.cell_color_blue = blue;
        self
    }

    /// Sets the alpha (transparency) component of the cell color in the display.
    pub fn cell_color_alpha(mut self, alpha: u8) -> Self {
        self.cell_color_alpha = alpha;
        self
    }

    /// Sets the background color of the display.
    pub fn background_color(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        self.background_color_red = red;
        self.background_color_green = green;
        self.background_color_blue = blue;
        self.background_color_alpha = alpha;
        self
    }

    /// Sets the red component of the background color in the display.
    pub fn background_color_red(mut self, red: u8) -> Self {
        self.background_color_red = red;
        self
    }

    /// Sets the green component of the background color in the display.
    pub fn background_color_green(mut self, green: u8) -> Self {
        self.background_color_green = green;
        self
    }

    /// Sets the blue component of the background color in the display.
    pub fn background_color_blue(mut self, blue: u8) -> Self {
        self.background_color_blue = blue;
        self
    }

    /// Sets the alpha (transparency) component of the background color in the display.
    pub fn background_color_alpha(mut self, alpha: u8) -> Self {
        self.background_color_alpha = alpha;
        self
    }

    /// Sets the color of the grid lines in the display.
    pub fn line_color(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        self.line_color_red = red;
        self.line_color_green = green;
        self.line_color_blue = blue;
        self.line_color_alpha = alpha;
        self
    }

    /// Sets the red component of the grid line color in the display.
    pub fn line_color_red(mut self, red: u8) -> Self {
        self.line_color_red = red;
        self
    }

    /// Sets the green component of the grid line color in the display.
    pub fn line_color_green(mut self, green: u8) -> Self {
        self.line_color_green = green;
        self
    }

    /// Sets the blue component of the grid line color in the display.
    pub fn line_color_blue(mut self, blue: u8) -> Self {
        self.line_color_blue = blue;
        self
    }

    /// Sets the alpha (transparency) component of the grid line color in the display.
    pub fn line_color_alpha(mut self, alpha: u8) -> Self {
        self.line_color_alpha = alpha;
        self
    }

    /// Sets the thickness of the grid lines in the display.
    pub fn line_thickness(mut self, line_thickness: u16) -> Self {
        self.line_thickness = line_thickness;
        self
    }

    /// Sets the number of rows in the simulation.
    pub fn rows(mut self, rows: u16) -> Self {
        self.rows = Some(rows);
        self
    }

    /// Sets the number of columns in the simulation.
    pub fn columns(mut self, columns: u16) -> Self {
        self.columns = Some(columns);
        self
    }

    /// Sets the surface type (affects wrapping) of the simulation.
    pub fn surface_type(mut self, surface_type: SurfaceType) -> Self {
        self.surface_type = surface_type;
        self
    }

    /// Sets the initial seed string for the simulation.
    pub fn seed(mut self, seed: &str) -> Self {
        self.seed = Some(String::from(seed));
        self
    }

    /// Sets the maximum number of generations to retain in the save history.
    pub fn maximum_saves(mut self, maximum_saves: u128) -> Self {
        self.maximum_saves = maximum_saves;
        self
    }

    /// Builds the `Simulation` instance based on the configured settings.
    pub fn build(self) -> Result<Simulation, String> {
        let (rows, columns, seed) = match (self.rows, self.columns, self.seed) {
            (Some(rows), Some(columns), Some(seed)) => (rows, columns, seed),
            (Some(rows), Some(columns), None) => (rows, columns, random_seed(rows, columns)),
            (Some(rows), None, Some(seed)) => {
                let seed_length = seed.len() as u16;
                if seed_length % rows == 0 {
                    (rows, seed_length / rows, seed)
                } else {
                    return Err(format!(
                        "The provided seed of \"{}\", must be divisible by the number of rows: {}",
                        seed, rows
                    ));
                }
            }
            (None, Some(columns), Some(seed)) => {
                let seed_length: u16 = seed.len() as u16;
                if seed_length % columns == 0 {
                    (seed_length / columns, columns, seed)
                } else {
                    return Err(format!(
                        "The provided seed of \"{}\", must be divisible by the number of columns: {}",
                        seed, columns
                    ));
                }
            }
            (None, None, Some(seed)) => {
                let seed_length: f32 = seed.len() as f32;
                let sqrt: f32 = seed_length.sqrt();
                let rounded_sqrt: f32 = sqrt.round();
                if (rounded_sqrt * rounded_sqrt) as usize == seed.len() {
                    let sqrt = rounded_sqrt as u16;
                    (sqrt, sqrt, seed)
                } else {
                    return Err(format!(
                        "The provided seed of \"{}\", must be of a square size (has an integer square root)",
                        seed
                    ));
                }
            }
            (Some(_), None, None) | (None, Some(_), None) => {
                return Err(
                    "Both rows and columns must be provided if no seed is provided".to_string(),
                );
            }
            (None, None, None) => {
                return Err(
                    "One of the following must be provided: rows, columns, or seed".to_string(),
                );
            }
        };

        let window_data: Option<SimulationWindowData> = if self.display {
            let (window_width, window_height, cell_width, cell_height) = match (
                self.window_width,
                self.window_height,
                self.cell_width,
                self.cell_height,
            ) {
                (Some(window_width), Some(window_height), None, None) => {
                    let cell_width: u16 = window_width / columns;
                    let cell_height: u16 = window_height / rows;
                    (window_width, window_height, cell_width, cell_height)
                }
                (None, None, Some(cell_width), Some(cell_height)) => {
                    let window_width: u16 = cell_width * columns;
                    let window_height: u16 = cell_height * rows;
                    (window_width, window_height, cell_width, cell_height)
                }
                (
                    Some(_window_width),
                    Some(_window_height),
                    Some(_cell_width),
                    Some(_cell_height),
                ) => {
                    return Err(
                        "Only cell dimensions or window dimensions can be provided, not both"
                            .to_string(),
                    );
                }
                _ => {
                    return Err(
                        "If the simulation has a display, a cell or window size must be provided"
                            .to_string(),
                    );
                }
            };
            Some(SimulationWindowData {
                window_width,
                window_height,
                window_title: self.window_title.clone(),
                cell_width,
                cell_height,
                window: Window::new(&*self.window_title, window_width, window_height),
                cell_color: (
                    self.cell_color_red,
                    self.cell_color_green,
                    self.cell_color_blue,
                    self.cell_color_alpha,
                ),
                background_color: (
                    self.background_color_red,
                    self.background_color_green,
                    self.background_color_blue,
                    self.background_color_alpha,
                ),
                line_color: (
                    self.line_color_red,
                    self.line_color_green,
                    self.line_color_blue,
                    self.line_color_alpha,
                ),
                line_thickness: self.line_thickness,
            })
        } else {
            None
        };
        let mut simulation = Simulation {
            seed: seed.clone(),
            surface_type: self.surface_type,
            rows,
            columns,
            generation: generation_from_string(seed, columns).unwrap(),
            generation_iteration: 0,
            save_history: Vec::new(),
            maximum_saves: self.maximum_saves,
            display: self.display,
            print: self.print,
            window_data,
        };
        simulation.draw_generation();
        Ok(simulation)
    }
}
