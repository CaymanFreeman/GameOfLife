use std::hash::Hash;
use crate::cell::CellState::*;

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum CellState {
    Dead,
    Alive,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Cell {
    pub state: CellState,
    pub row: u16,
    pub column: u16,
}

pub const ALIVE_CHAR: char = '*';
pub const DEAD_CHAR: char = '-';

impl Cell {

    pub fn is_alive(&self) -> bool {
        if self.state == Alive {
            return true
        }
        return false
    }

    pub fn as_char(&self) -> char {
        match self.state {
            Alive => { ALIVE_CHAR }
            Dead => { DEAD_CHAR }
        }
    }

    pub fn new(state: CellState, row: u16, column: u16) -> Result<Cell, String> {
        Ok(Cell {
            state,
            row,
            column
        })
    }

    pub fn new_alive(row: u16, column: u16) -> Cell { Self::new(Alive, row, column).unwrap() }
}