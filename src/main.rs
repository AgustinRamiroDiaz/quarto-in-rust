use std::fmt::Debug;

fn main() {
    println!("{:#?}", empty_grid::<Piece>());
}

const QUATRO: usize = 4;
const N_PROPERTIES: usize = 4;
const BOARD_SIZE: usize = 4;

type Piece = [bool; N_PROPERTIES];

type Grid<T> = [[Option<T>; BOARD_SIZE]; BOARD_SIZE];

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

struct GameState {
    player_turn: PlayerTurn,
    stage: Stage,
}

enum PlayerTurn {
    Player1,
    Player2,
}

#[derive(PartialEq)]
enum Stage {
    ChoosingPieceForOponent,
    PlacingPieceGivenOponentChoice,
}
struct Game {
    board: Board<Piece>,
    game_state: GameState,
}

impl Game {
    fn new() -> Game {
        Game {
            board: Board::new(),
            game_state: GameState {
                player_turn: PlayerTurn::Player1,
                stage: Stage::ChoosingPieceForOponent,
            },
        }
    }

    fn check_if_won(&self, position: Coordinate) -> bool {
        let rowItems = self.board.grid[position.row]
            .into_iter()
            .flatten()
            .collect::<Vec<Piece>>();
        if rowItems.len() == BOARD_SIZE && check_match(rowItems) {
            return true;
        }

        let column_items = self
            .board
            .grid
            .iter()
            .filter_map(|row| row[position.column])
            .collect::<Vec<Piece>>();

        if column_items.len() == QUATRO && check_match(column_items) {
            return true;
        }

        let backward_slash_diagonal = (0..BOARD_SIZE)
            .filter_map(|n| self.board.grid[n][n])
            .collect::<Vec<Piece>>();

        if backward_slash_diagonal.len() == QUATRO && check_match(backward_slash_diagonal) {
            return true;
        }

        let forward_slash_diagonal = (0..BOARD_SIZE)
            .filter_map(|n| self.board.grid[n][BOARD_SIZE - n])
            .collect::<Vec<Piece>>();

        if forward_slash_diagonal.len() == QUATRO && check_match(forward_slash_diagonal) {
            return true;
        }

        false
    }

    fn put(&mut self, piece: Piece, position: Coordinate) -> Result<(), String> {
        if self.game_state.stage == Stage::ChoosingPieceForOponent {
            return Err("You can't place a piece during the choosing piece stage".to_string());
        }
        self.board.put(piece, position)?;
        // TODO: non board pieces
        // check that the piece is valid
        // TODO: add player as parameter and check
        self.game_state.stage = Stage::PlacingPieceGivenOponentChoice;
        self.game_state.player_turn = match self.game_state.player_turn {
            PlayerTurn::Player1 => PlayerTurn::Player2,
            PlayerTurn::Player2 => PlayerTurn::Player1,
        };

        Ok(())
    }

    fn put_and_check_if_won(&mut self, piece: Piece, position: Coordinate) -> Result<bool, String> {
        self.put(piece, position)?;
        Ok(self.check_if_won(position))
    }
}

fn check_match(pieces: Vec<Piece>) -> bool {
    for property in 0..QUATRO {
        let properties = pieces
            .iter()
            .map(|piece| piece[property])
            .collect::<Vec<bool>>();

        let first_property = properties[0];
        if properties
            .iter()
            .all(|property| property == &first_property)
        {
            return true;
        }
    }

    false
}
