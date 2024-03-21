use std::hash::Hash;
use num_traits::Unsigned;
use crate::cell::CellState::*;

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum CellState {
    Dead,
    Alive,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Cell<U: Unsigned + Eq + Hash> {
    pub state: CellState,
    pub row: U,
    pub column: U,
}

pub const ALIVE_DISPLAY: char = '*';
pub const DEAD_DISPLAY: char = '-';

pub const ALIVE_SEED_VALUE: char = '1';
pub const DEAD_SEED_VALUE: char = '0';

impl<U: Unsigned + Eq + Hash> Cell<U> {

    pub fn is_alive(&self) -> bool {
        if self.state == Alive {
            return true
        }
        return false
    }

    pub fn to_seed_value(&self) -> char {
        match self.state {
            Alive => { ALIVE_SEED_VALUE }
            Dead => { DEAD_SEED_VALUE }
        }
    }

    pub fn to_display(&self) -> char {
        match self.state {
            Alive => { ALIVE_DISPLAY }
            Dead => { DEAD_DISPLAY }
        }
    }

    pub fn new(state: CellState, row: U, column: U) -> Result<Cell<U>, String> {
        Ok(Cell {
            state,
            row,
            column
        })
    }

    pub fn new_alive(row: U, column: U) -> Cell<U> { Self::new(Alive, row, column).unwrap() }
}