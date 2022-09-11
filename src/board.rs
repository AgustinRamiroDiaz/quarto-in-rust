use super::coordinate::Coordinate;

use std::fmt::Debug;

use super::QUATRO;

use super::BOARD_SIZE;
use serde::{Deserialize, Serialize};

pub(crate) type Grid<T> = [[Option<T>; BOARD_SIZE]; BOARD_SIZE];

pub(crate) fn empty_row<T>() -> [Option<T>; QUATRO] {
    [None, None, None, None]
}

pub(crate) fn empty_grid<T>() -> Grid<T> {
    [empty_row(), empty_row(), empty_row(), empty_row()]
}

#[derive(Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Board<T> {
    pub(crate) grid: Grid<T>,
}

impl<T: Copy + Debug> Board<T> {
    pub(crate) fn new() -> Self {
        Board { grid: empty_grid() }
    }

    pub(crate) fn get(&self, position: Coordinate) -> Result<Option<T>, String> {
        if position.row > QUATRO || position.column > QUATRO {
            return Err(format!(
                "Position out of bounds: you requested {position:#?} but board size is {QUATRO}"
            ));
        }
        Ok(self.grid[position.row][position.column])
    }

    pub(crate) fn put(&mut self, piece: T, position: Coordinate) -> Result<(), String> {
        match self.get(position)? {
            Some(piece) => Err(format!("Place occupied by {piece:#?}")),
            None => {
                self.grid[position.row][position.column] = Some(piece);
                Ok(())
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) fn remove(&mut self, position: Coordinate) -> Result<T, String> {
        match self.get(position)? {
            Some(piece) => {
                self.grid[position.row][position.column] = None;
                Ok(piece)
            }
            None => Err("Place is empty".to_string()),
        }
    }

    pub(crate) fn empty_spaces(&self) -> Vec<Coordinate> {
        let mut result = Vec::new();

        for (row_index, row) in self.grid.iter().enumerate() {
            for (column_index, value) in row.iter().enumerate() {
                match value {
                    None => result.push(Coordinate {
                        column: column_index,
                        row: row_index,
                    }),
                    Some(_) => (),
                }
            }
        }

        result
    }
}
