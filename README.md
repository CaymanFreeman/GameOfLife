# Game of Life

This library assists with creating simple Game of Life simulations. It is based on [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway's_Game_of_Life), invented by John Conway in 1970. The main difference with this library is that there is no infinite plane, but four different finite surfaces defined through the `SurfaceType` enum. This is my first Rust project and picked simulating the Game of Life to learn the language.

To use this library, add it to the dependency section to the `Cargo.toml` file for your project as shown below. You can pick a specific version or use '*' for the latest.
```TOML {id="adding-to-project" data-filename="readme.md"}
[dependencies]
simple_game_of_life = "*"
```

## Dependencies

This library depends on the [simple](https://docs.rs/simple/latest/simple/index.html) graphics library, which itself depends on [SDL](https://github.com/libsdl-org/SDL) and [SDL Image](https://github.com/libsdl-org/SDL_image). You will need to install these if you would like to use display windows.

## Documentation
The documentation for this project is compiled with [rustdoc](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html) and can be found at the [package's website](https://docs.rs/simple_game_of_life/1.0.0/simple_game_of_life/).

## Getting Started

This is a simple 5x5 simulation with a display and a demonstration for continuous simulation generation.

```Rust {id="getting-started" data-filename="readme.md"}
use std::time::Duration;
use game_of_life::simulation::{Simulation, SurfaceType};
use game_of_life::simulation_builder::SimulationBuilder;

let mut simulation: Simulation = SimulationBuilder::new()
    .rows(5) // 5 rows high
    .columns(5) // 5 columns wide
    .surface_type(SurfaceType::Rectangle) // Rectangle (non-wrapping) surface
    .display(true) // Declaring that the simulation should display the generations in a window
    .cell_size(50) // Cell size of 50x50 pixels
    .build() // Build into a simulation
    .unwrap();

// This will run the entire simulation with a display window,
// updating the display with each generation every 250 milliseconds
// until it detects a still or periodic simulation
simulation.simulate_continuous_generations(Duration::from_millis(250), true)
```

## Surface Types

Each of these examples will use the same 7x7 seed with a window display to show an example of how they function.

### Rectangle

The [Rectangle](https://docs.rs/simple_game_of_life/1.0.0/simple_game_of_life/simulation/enum.SurfaceType.html#variant.Rectangle) is the simplest surface type where there is no wrapping, which means all edges are "dead zones".

![Rectangle Surface Demonstration GIF](https://i.imgur.com/Z7Lyseq.gif)

### Ball

The [Ball](https://docs.rs/simple_game_of_life/1.0.0/simple_game_of_life/simulation/enum.SurfaceType.html#variant.Ball) is a surface type where there are no "dead zones". Every side of the simulation will wrap around to the opposite side.

![Ball Surface Demonstration GIF](https://i.imgur.com/bO1AHsA.gif)

### Horizontal Loop

The [Horizontal Loop](https://docs.rs/simple_game_of_life/1.0.0/simple_game_of_life/simulation/enum.SurfaceType.html#variant.HorizontalLoop) is a surface type where the top and bottom of the simulation are "dead zones" and the left and right will wrap around to each other. This is the same behavior as the video game [Pac-Man](https://en.wikipedia.org/wiki/Pac-Man).

![Horizontal Loop Surface Demonstration GIF](https://i.imgur.com/rR0HQiE.gif)

### Vertical Loop

The [Vertical Loop](https://docs.rs/simple_game_of_life/1.0.0/simple_game_of_life/simulation/enum.SurfaceType.html#variant.VerticalLoop) is a surface type where the left and right of the simulation are "dead zones" and the top and bottom will wrap around to each other.

![Vertical Loop Surface Demonstration GIF](https://i.imgur.com/yKB6Azk.gif)

## Display Types & Customization

### Printing

The simplest and minimal option for viewing the simulation is through terminal printing. Simulations implement [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html) so they can easily be printed. Simulations don't need the `.print(true)` flag to print, but it is needed if you want the simulation to print automatically each time a generation is simulated.

```Rust {id="simulation-printing" data-filename="readme.md"}
println!("{}", simulation)
```

```Terminal
1
-----
--**-
-*-*-
--**-
-----
```

### Display Windows

There is also the option to display the simulation in a more colorful way in a window like you've seen in the demonstrations. Unlike printing, the `.display(true)` flag is required to view a simulation with a window. After each iteration of a simulation, the window will automatically update the next frame to display the current generation of cells.

The window display is customizable through the different color and size options. Each customization flag can be viewed on the `SimulationBuilder`'s [documentation page](https://docs.rs/simple_game_of_life/1.0.0/simple_game_of_life/simulation_builder/struct.SimulationBuilder.html), but there are some examples of what you can do below.

```Rust {id="customization-demonstration-1" data-filename="readme.md"}
.cell_color(255, 0, 0, 255) // Red cells
.background_color(0, 0, 0, 255) // Black background
```

![Red and Black Example](https://i.imgur.com/rw45eqD.gif)

```Rust {id="customization-demonstration-2" data-filename="readme.md"}
.cell_color(0, 255, 20, 255) // Green cells
.line_color(0, 20, 200, 255) // Blue lines
```

![Green and Blue Example](https://i.imgur.com/LXZdFaT.gif)

```Rust {id="customization-demonstration-3" data-filename="readme.md"}
.cell_width(50) // 50px cell width
.cell_height(85) // 85px cell height
```

![Stretched Example](https://i.imgur.com/Xfy5L2G.gif)
