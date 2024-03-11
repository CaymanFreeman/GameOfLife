use crate::cell::CellState::*;

#[derive(Clone, Eq, PartialEq, Hash)]
pub(crate) enum CellState {
    Dead,
    Alive,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub(crate) struct Cell {
    pub(crate) state: CellState,
    pub(crate) row: i32,
    pub(crate) column: i32,
}

pub(crate) const ALIVE_DISPLAY: char = '*';
pub(crate) const DEAD_DISPLAY: char = '-';

pub(crate) const ALIVE_SEED_VALUE: char = '1';
pub(crate) const DEAD_SEED_VALUE: char = '0';

impl Cell {

    pub(crate) fn is_alive(&self) -> bool {
        if self.state == Alive {
            return true
        }
        return false
    }

    pub(crate) fn to_seed_value(&self) -> char {
        match self.state {
            Alive => { ALIVE_SEED_VALUE }
            Dead => { DEAD_SEED_VALUE }
        }
    }

    pub(crate) fn to_display(&self) -> char {
        match self.state {
            Alive => { ALIVE_DISPLAY }
            Dead => { DEAD_DISPLAY }
        }
    }

    pub(crate) fn new(state: CellState, row: i32, column: i32) -> Result<Cell, String> {
        Ok(Cell {
            state,
            row,
            column
        })
    }

    pub(crate) fn new_alive(row: i32, column: i32) -> Cell { Self::new(Alive, row, column).unwrap() }
}