use crate::cell::CellState::{ALIVE, DEAD};

/// Represents the state of a cell.
#[derive(Clone, Eq, PartialEq, Hash)]
pub(crate) enum CellState {
    /// A dead cell.
    DEAD,
    /// An alive cell.
    ALIVE,
}

/// Represents a single cell in a `Simulation`.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Cell {
    /// The state of the cell (alive or dead).
    pub(crate) state: CellState,
    /// The row index of the cell.
    pub(crate) row: u16,
    /// The column index of the cell.
    pub(crate) column: u16,
}

/// The character that represents a cell with an `Alive` `CellState` in string
/// representations of a generation.
pub const ALIVE_CHAR: char = '*';
/// The character that represents a cell with a `Dead` `CellState` in string
/// representations of a generation.
pub const DEAD_CHAR: char = '-';

impl Cell {
    /// Returns true if the cell is alive, false otherwise.
    pub(crate) fn is_alive(&self) -> bool {
        if self.state == ALIVE {
            return true;
        }
        return false;
    }

    /// Returns the character representation of the cell's state.
    pub(crate) fn as_char(&self) -> char {
        match self.state.clone() {
            ALIVE => ALIVE_CHAR,
            DEAD => DEAD_CHAR,
        }
    }

    /// Creates a new `Cell` instance with the given state, row, and column.
    pub(crate) fn new(state: CellState, row: u16, column: u16) -> Cell {
        Cell { state, row, column }
    }
}
