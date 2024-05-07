//! GameOfLife
//! ======
//!
//! A simple Game of Life library built with Rust.
//!
//! GitHub: <https://github.com/CaymanFreeman/GameOfLife>
//!
//! # Getting Started
//!
//! ```rust,no_run
//! use std::time::Duration;
//! use simple_game_of_life::simulation::{Simulation, SurfaceType};
//! use simple_game_of_life::simulation_builder::SimulationBuilder;
//!
//! let mut simulation: Simulation = SimulationBuilder::new()
//!     .rows(4) // 4 rows high
//!     .columns(9) // 9 columns wide
//!     .surface_type(SurfaceType::Rectangle) // Rectangle (non-wrapping) surface
//!     .display(true) // Declaring that the simulation should display the generations in a window
//!     .cell_size(50) // Cell size of 50x50 pixels
//!     .build() // Build into a simulation
//!     .unwrap();
//!
//! // This will run the entire simulation with a display window,
//! // updating the display with each generation every 250 milliseconds
//! // until it detects a still or periodic simulation
//! simulation.simulate_continuous_generations(Duration::from_millis(250), true)
//! ```

extern crate core;
extern crate rand;
extern crate simple;

pub(crate) mod cell;
pub mod simulation;
pub mod simulation_builder;
pub(crate) mod simulation_window;
