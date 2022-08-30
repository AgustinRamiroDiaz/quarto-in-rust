use std::fmt::Debug;

fn main() {
    println!("{:#?}", empty_grid::<Piece>());
}

const QUATRO: usize = 4;

type Piece = [bool; QUATRO];

type Grid<T> = [[Option<T>; QUATRO]; QUATRO];

fn empty_row<T>() -> [Option<T>; QUATRO] {
    [None, None, None, None]
}

fn empty_grid<T>() -> Grid<T> {
    [empty_row(), empty_row(), empty_row(), empty_row()]
}

struct Board<T> {
    grid: Grid<T>,
}

#[derive(Debug, Copy, Clone)]
struct Coordinate {
    row: usize,
    column: usize,
}

impl<T: Copy + Debug> Board<T> {
    fn new() -> Self {
        Board { grid: empty_grid() }
    }

    fn get(&self, position: Coordinate) -> Result<Option<T>, String> {
        if position.row > QUATRO || position.column > QUATRO {
            return Err(format!(
                "Position out of bounds: you requested {position:#?} but board size is {QUATRO}"
            ));
        }
        Ok(self.grid[position.column][position.row])
    }

    fn put(&mut self, piece: T, position: Coordinate) -> Result<(), String> {
        match self.get(position)? {
            Some(piece) => Err(format!("Place occupied by {piece:#?}")),
            None => {
                self.grid[position.column][position.row] = Some(piece);
                Ok(())
            }
        }
    }

    fn remove(&mut self, position: Coordinate) -> Result<(), String> {
        match self.get(position)? {
            Some(_) => {
                self.grid[position.column][position.row] = None;
                Ok(())
            }
            None => Err("Place is empty".to_string()),
        }
    }

    fn empty_spaces(&self) -> Vec<Coordinate> {
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

struct Game {
    board: Board<Piece>,
}

impl Game {
    fn new() -> Game {
        Game {
            board: Board::new(),
        }
    }

    fn check_if_won(&self, position: Coordinate) -> bool {
        let rowItems = self.board.grid[position.row]
            .into_iter()
            .flatten()
            .collect::<Vec<Piece>>();
        if rowItems.len() == QUATRO {
            //     if
        }

        false
    }

    fn put_and_check_if_won(&mut self, piece: Piece, position: Coordinate) -> Result<bool, String> {
        self.board.put(piece, position)?;
        Ok(self.check_if_won(position))
    }
}
