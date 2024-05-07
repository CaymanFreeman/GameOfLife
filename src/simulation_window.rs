use crate::simulation::Simulation;
use simple::{Rect, Window};
use std::thread::sleep;
use std::time::{Duration, Instant};

/// Represents the data related to the display window for the simulation.
pub(crate) struct SimulationWindowData {
    /// The window object used for rendering the simulation.
    pub(crate) window: Window,
    /// The width of the display window in pixels.
    pub(crate) window_width: u16,
    /// The height of the display window in pixels.
    pub(crate) window_height: u16,
    /// The title of the display window.
    pub(crate) window_title: String,
    /// The width of each cell in the display in pixels.
    pub(crate) cell_width: u16,
    /// The height of each cell in the display in pixels.
    pub(crate) cell_height: u16,
    /// The color of the cells in the display, represented as an RGBA tuple.
    pub(crate) cell_color: (u8, u8, u8, u8),
    /// The background color of the display, represented as an RGBA tuple.
    pub(crate) background_color: (u8, u8, u8, u8),
    /// The color of the grid lines in the display, represented as an RGBA tuple.
    pub(crate) line_color: (u8, u8, u8, u8),
    /// The thickness of the grid lines in the display in pixels.
    pub(crate) line_thickness: u16,
}

impl Clone for SimulationWindowData {
    /// Creates a deep clone of the `SimulationWindowData` instance.
    fn clone(&self) -> Self {
        SimulationWindowData {
            window_width: self.window_width,
            window_height: self.window_height,
            window_title: self.window_title.clone(),
            window: Window::new(&*self.window_title, self.window_width, self.window_height),
            cell_width: self.cell_width,
            cell_height: self.cell_height,
            cell_color: self.cell_color,
            background_color: self.background_color,
            line_color: self.line_color,
            line_thickness: self.line_thickness,
        }
    }
}

impl Simulation {
    /// Draws the grid lines representing the cell boundaries on the simulation display window.
    ///
    /// # Description
    /// This function is responsible for rendering the grid lines that separate the individual
    /// cells in the simulation display window. The grid lines are drawn using the specified
    /// line color and thickness.
    ///
    /// The grid lines are drawn as vertical and horizontal lines based on the number of rows
    /// and columns in the simulation. The vertical lines are drawn between each column, while
    /// the horizontal lines are drawn between each row.
    ///
    /// This function should be called after the alive cells have been drawn to ensure that the
    /// grid lines are visible on top of the cells.
    fn draw_cell_grid(&mut self) {
        let window_data: &mut SimulationWindowData = self.window_data.as_mut().unwrap();
        let line_color: (u8, u8, u8, u8) = window_data.line_color;
        window_data
            .window
            .set_color(line_color.0, line_color.1, line_color.2, line_color.3);
        let cell_width: u16 = window_data.cell_width;
        let cell_height: u16 = window_data.cell_height;
        for column in 1..self.columns {
            window_data.window.fill_rect(Rect::new(
                ((column * cell_width) - (window_data.line_thickness / 2)) as i32,
                0,
                window_data.line_thickness as u32,
                window_data.window_height as u32,
            ));
        }
        for row in 1..self.rows {
            window_data.window.fill_rect(Rect::new(
                0,
                ((row * cell_height) - (window_data.line_thickness / 2)) as i32,
                window_data.window_width as u32,
                window_data.line_thickness as u32,
            ));
        }
    }

    /// Draws the alive cells on the simulation display window.
    ///
    /// # Description
    /// This function iterates through the current generation of cells and draws each alive cell
    /// on the simulation display window.
    ///
    /// The alive cells are represented as filled rectangles using the specified cell color.
    ///
    /// Before drawing the alive cells, the background of the display window is filled with the
    /// specified background color to clear any previously drawn cells or grid lines.
    ///
    /// The position and size of each drawn cell are determined by the row and column indices of
    /// the cell, combined with the specified cell width and height.
    ///
    /// This function should be called before drawing the grid lines to ensure that the alive
    /// cells are visible underneath the grid lines.
    fn draw_alive_cells(&mut self) {
        let window_data: &mut SimulationWindowData = self.window_data.as_mut().unwrap();
        let background_color: (u8, u8, u8, u8) = window_data.background_color;
        window_data.window.set_color(
            background_color.0,
            background_color.1,
            background_color.2,
            background_color.3,
        );
        window_data.window.fill_rect(Rect::new(
            0,
            0,
            window_data.window_width as u32,
            window_data.window_height as u32,
        ));
        let cell_color: (u8, u8, u8, u8) = window_data.cell_color;
        window_data
            .window
            .set_color(cell_color.0, cell_color.1, cell_color.2, cell_color.3);
        let cell_width: u16 = window_data.cell_width;
        let cell_height: u16 = window_data.cell_height;
        for cell in &self.generation {
            if cell.is_alive() {
                let x: i32 = (cell.column * cell_width) as i32;
                let y: i32 = (cell.row * cell_height) as i32;
                window_data.window.fill_rect(Rect::new(
                    x,
                    y,
                    cell_width as u32,
                    cell_height as u32,
                ));
            }
        }
    }

    /// Draws the current generation of cells on the simulation display window.
    ///
    /// # Description
    /// This function combines the functionality of `draw_alive_cells` and `draw_cell_grid` to
    /// render the complete visualization of the current generation.
    ///
    /// First, `draw_alive_cells` is called to draw all the alive cells on the display window
    /// using the specified cell color and background color.
    ///
    /// Next, `draw_cell_grid` is called to draw the grid lines separating the individual cells,
    /// using the specified line color and thickness.
    ///
    /// After both the alive cells and grid lines have been drawn, the `next_frame` method of the
    /// display window is called to update the window with the new frame.
    ///
    /// This function is called whenever the simulation generation changes to update the
    /// visualization in the display window.
    pub(crate) fn draw_generation(&mut self) {
        self.draw_alive_cells();
        self.draw_cell_grid();
        self.window_data.as_mut().unwrap().window.next_frame();
    }

    /// Freezes the simulation window indefinitely to keep the current generation displayed.
    pub fn freeze_window(&mut self) {
        loop {
            self.window_data.as_mut().unwrap().window.next_frame();
            sleep(Duration::from_millis(100));
        }
    }

    /// Freezes the simulation window for the specified duration to keep the current
    /// generation displayed.
    pub fn freeze_window_for(&mut self, duration: Duration) {
        let start_time = Instant::now();
        loop {
            if Instant::now().duration_since(start_time) >= duration {
                break;
            }
            self.window_data.as_mut().unwrap().window.next_frame();
            sleep(Duration::from_millis(100));
        }
    }
}
