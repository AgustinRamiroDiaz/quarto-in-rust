use super::coordinate::Coordinate;

use std::convert::TryInto;
use std::fmt::Debug;

use super::BOARD_SIZE;
use super::QUATRO;
use ndarray::Array2;
use serde::{Deserialize, Serialize};

pub(crate) type Grid<T> = [[Option<T>; BOARD_SIZE]; BOARD_SIZE];

pub(crate) fn empty_row<T>() -> [Option<T>; QUATRO] {
    [None, None, None, None]
}

pub(crate) fn empty_grid<T>() -> Array2<Option<T>>
where
    T: Copy,
{
    Array2::from_elem((BOARD_SIZE, BOARD_SIZE), None)
}

#[derive(Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Board<T> {
    pub(crate) grid: Array2<Option<T>>,
}

impl<T: Copy + Debug> Board<T> {
    pub(crate) fn new() -> Self {
        Board { grid: empty_grid() }
    }

    pub(crate) fn get(&self, position: Coordinate) -> Result<Option<T>, String> {
        if position.row > QUATRO || position.column > QUATRO {
            return Err(format!(
                "Position out of bounds: you requested {position:#?} but board size is {BOARD_SIZE}"
            ));
        }
        match self.grid.get((position.row, position.column)) {
            Some(value) => Ok(*value),
            None => Err(format!(
                "Position out of bounds: you requested {position:#?} but board size is {BOARD_SIZE}"
            )),
        }
    }

    pub(crate) fn get_row(&self, row: usize) -> Result<Vec<Option<T>>, String> {
        if row > self.grid.nrows() {
            return Err(format!(
                "Row out of bounds: you requested row {row} but board size is {BOARD_SIZE}"
            ));
        }
        Ok(self.grid.row(row).to_vec())
    }

    pub(crate) fn get_column(&self, column: usize) -> Result<Vec<Option<T>>, String> {
        if column > self.grid.ncols() {
            return Err(format!(
                "Column out of bounds: you requested column {column} but board size is {BOARD_SIZE}"
            ));
        }
        Ok(self.grid.column(column).to_vec())
    }

    pub(crate) fn get_diagonal(&self) -> Vec<Option<T>> {
        self.grid.diag().to_vec()
    }

    pub(crate) fn put(&mut self, piece: T, position: Coordinate) -> Result<(), String> {
        match self.get(position)? {
            Some(piece) => Err(format!("Place occupied by {piece:#?}")),
            None => {
                match self.grid.get_mut((position.row, position.column)) {
                    None => Err(format!(
                        "Position out of bounds: you requested {position:#?} but board size is {BOARD_SIZE}"
                    )),
                    Some(value) => {
                        *value = Some(piece);
                        Ok(())
                    }
                }
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) fn remove(&mut self, position: Coordinate) -> Result<T, String> {
        match self.get(position)? {
            Some(piece) => {
                match self.grid.get_mut((position.row, position.column)) {
                    None => Err(format!(
                        "Position out of bounds: you requested {position:#?} but board size is {BOARD_SIZE}"
                    )),
                    Some(value) => {
                        *value = None;
                        Ok(piece)
                    }
                }
            }
            None => Err("Place is empty".to_string()),
        }
    }

    pub(crate) fn empty_spaces(&self) -> Vec<Coordinate> {
        self.grid
            .indexed_iter()
            .filter_map(|((row, column), value)| match value {
                Some(_) => None,
                None => Some(Coordinate { row, column }),
            })
            .collect()
    }
}
