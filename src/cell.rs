use crate::cell::CellState::*;

#[derive(PartialEq, Clone)]
pub(crate) enum CellState {
    Dead,
    Alive,
}

#[derive(Clone)]
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

    pub(crate) fn new(state: CellState, row: i32, column: i32) -> Cell {
        return Cell {
            state,
            row,
            column
        }
    }
}