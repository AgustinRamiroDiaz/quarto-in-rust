use std::{
    collections::HashSet,
    fmt::{self, Debug},
};

fn main() -> Result<(), String> {
    let mut game = Game::new();
    // for (piece, coordinate) in game
    //     .get_pieces_left()
    //     .iter()
    //     .zip(game.get_empty_places().iter())
    // {
    //     game.choose(*piece)?;
    //     game.put(*coordinate)?;
    // }

    let qmm = QuatoMinimax::new();
    let initial_state = &Game::new();
    let actions = qmm.actions(initial_state);
    let actions_with_values = actions
        .iter()
        .map(|action| (action, qmm.max_value(initial_state)))
        .collect::<Vec<_>>();

    print!("{:#?}", actions_with_values);

    println!("BOARD\n{}\nBOARD", game.board);
    Ok(())
}

const QUATRO: usize = 4;
const N_PROPERTIES: usize = 4;
const BOARD_SIZE: usize = 4;

type Piece = [bool; N_PROPERTIES];

fn all_possible_pieces(size: usize) -> Vec<Vec<bool>> {
    if size == 0 {
        return vec![vec![]];
    }
    let smaller_pieces = all_possible_pieces(size - 1);
    let mut pieces = vec![];
    for piece in smaller_pieces {
        let mut new_piece = piece.clone();
        new_piece.push(false);
        pieces.push(new_piece);
        let mut new_piece = piece.clone();
        new_piece.push(true);
        pieces.push(new_piece);
    }
    pieces
}

type Grid<T> = [[Option<T>; BOARD_SIZE]; BOARD_SIZE];

fn empty_row<T>() -> [Option<T>; QUATRO] {
    [None, None, None, None]
}

fn empty_grid<T>() -> Grid<T> {
    [empty_row(), empty_row(), empty_row(), empty_row()]
}

#[derive(Debug)]
struct Board<T> {
    grid: Grid<T>,
}

impl Clone for Board<Piece> {
    fn clone(&self) -> Self {
        Board {
            grid: self.grid.clone(),
        }
    }
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

impl<T: Debug + Copy> fmt::Display for Board<T> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self
            .grid
            .map(|row| {
                row.map(|cell| match cell {
                    Some(piece) => format!("{:?}", piece),
                    None => "#".to_string(),
                })
                .join("\t")
            })
            .join("\n");
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug)]
struct GameState {
    player_turn: PlayerTurn,
    stage: Stage,
}

#[derive(Clone, Debug)]
enum PlayerTurn {
    Player1,
    Player2,
}

#[derive(PartialEq, Clone, Debug)]
enum Stage {
    ChoosingPieceForOponent,
    PlacingPieceGivenOponentChoice(Piece),
}
#[derive(Clone, Debug)]
struct Game {
    board: Board<Piece>,
    game_state: GameState,
    pieces_left: HashSet<Piece>,
}

impl Game {
    fn new() -> Game {
        let mut pieces = HashSet::<Piece>::new();
        for piece in all_possible_pieces(N_PROPERTIES) {
            pieces.insert(piece.try_into().unwrap()); // TODO: may panic
        }

        Game {
            board: Board::new(),
            game_state: GameState {
                player_turn: PlayerTurn::Player1,
                stage: Stage::ChoosingPieceForOponent,
            },
            pieces_left: pieces,
        }
    }

    fn check_row_match(&self, row: usize) -> bool {
        let row_items = self.board.grid[row]
            .into_iter()
            .flatten()
            .collect::<Vec<Piece>>();
        row_items.len() == BOARD_SIZE && check_match(row_items)
    }

    fn check_column_match(&self, column: usize) -> bool {
        let column_items = self
            .board
            .grid
            .iter()
            .filter_map(|row| row[column])
            .collect::<Vec<Piece>>();

        column_items.len() == QUATRO && check_match(column_items)
    }

    fn check_backward_slash_diagonal(&self) -> bool {
        let backward_slash_diagonal = (0..BOARD_SIZE)
            .filter_map(|n| self.board.grid[n][n])
            .collect::<Vec<Piece>>();

        backward_slash_diagonal.len() == QUATRO && check_match(backward_slash_diagonal)
    }

    fn check_forward_slash_diagonal(&self) -> bool {
        let forward_slash_diagonal = (0..BOARD_SIZE)
            .filter_map(|n| self.board.grid[n][BOARD_SIZE - n])
            .collect::<Vec<Piece>>();

        forward_slash_diagonal.len() == QUATRO && check_match(forward_slash_diagonal)
    }

    fn check_if_won(&self, position: Coordinate) -> bool {
        if self.check_row_match(position.row) {
            return true;
        }

        if self.check_column_match(position.column) {
            return true;
        }

        if position.row == position.column && self.check_backward_slash_diagonal() {
            return true;
        }

        if position.row + position.column == QUATRO - 1 && self.check_forward_slash_diagonal() {
            return true;
        }

        false
    }

    fn get_pieces_left(&self) -> Vec<Piece> {
        self.pieces_left.iter().cloned().collect()
    }

    fn get_empty_places(&self) -> Vec<Coordinate> {
        self.board.empty_spaces()
    }

    fn choose(&mut self, piece: Piece) -> Result<(), String> {
        // TODO: add player as parameter and check

        if !self.get_pieces_left().contains(&piece) {
            // TODO: this may not work due to reference
            return Err("Piece not available".to_string());
        }

        match self.game_state.stage {
            Stage::PlacingPieceGivenOponentChoice(_) => {
                Err("You can't place a piece right now".to_string())
            }
            Stage::ChoosingPieceForOponent => {
                self.pieces_left.remove(&piece); // TODO: this may not work due to reference

                self.game_state.stage = Stage::PlacingPieceGivenOponentChoice(piece);
                self.game_state.player_turn = match self.game_state.player_turn {
                    PlayerTurn::Player1 => PlayerTurn::Player2,
                    PlayerTurn::Player2 => PlayerTurn::Player1,
                };
                Ok(())
            }
        }
    }

    fn put(&mut self, position: Coordinate) -> Result<(), String> {
        // TODO: add player as parameter and check
        match self.game_state.stage {
            Stage::ChoosingPieceForOponent => {
                return Err("You can't place a piece right now".to_string())
            }
            Stage::PlacingPieceGivenOponentChoice(piece) => {
                self.board.put(piece, position)?;
                self.game_state.stage = Stage::ChoosingPieceForOponent;
                self.game_state.player_turn = match self.game_state.player_turn {
                    PlayerTurn::Player1 => PlayerTurn::Player2,
                    PlayerTurn::Player2 => PlayerTurn::Player1,
                };

                Ok(())
            }
        }
    }

    fn put_and_check_if_won(&mut self, position: Coordinate) -> Result<bool, String> {
        self.put(position)?;
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

// Minimax
// fn max_value(state):
//     if terminal(state):
//         return utility(state)
//     v = -infinity
//     for each action in actions(state):
//         v = max(v, min_value(result(state, action)))
//     return v

// fn min_value(state):
//     if terminal(state):
//         return utility(state)
//     v = infinity
//     for each action in actions(state):
//         v = min(v, max_value(result(state, action)))
//     return v

trait Minimax<State, Action>
where
    State: Clone,
{
    fn utility(&self, state: &State) -> i32;
    fn terminal(&self, state: &State) -> bool;
    fn actions(&self, state: &State) -> Vec<Action>;
    fn result(&self, state: &State, action: Action) -> State;

    fn min_value(&self, state: &State) -> i32 {
        let cloned_state = state.clone();
        if self.terminal(&cloned_state) {
            return self.utility(&cloned_state);
        }

        let mut v = i32::MAX;
        for action in self.actions(&cloned_state) {
            v = v.min(self.max_value(&self.result(&cloned_state, action)));
        }

        v
    }
    fn max_value(&self, state: &State) -> i32 {
        let cloned_state = state.clone();
        if self.terminal(&cloned_state) {
            return self.utility(&cloned_state);
        }

        let mut v = i32::MIN;
        for action in self.actions(&cloned_state) {
            v = v.max(self.min_value(&self.result(&cloned_state, action)));
        }

        v
    }
}

struct QuatoMinimax {}

#[derive(Copy, Clone, Debug)]
enum QuatroAction {
    Choose(Piece),
    Put(Coordinate),
}

impl QuatoMinimax {
    fn new() -> QuatoMinimax {
        QuatoMinimax {}
    }
}

impl Minimax<Game, QuatroAction> for QuatoMinimax {
    // We'll take into account the perspective of player 1 to calculate the utility
    // This function only makes sense for terminal states
    fn utility(&self, state: &Game) -> i32 {
        // TODO: decouple this from "Game" state management
        // this depends on the state handling of "Game"
        // at the moment, every time someone puts a piece the turn changes
        // so if the turn is player 1, it means that player 2 just put a piece
        // and if the turn is player 2, it means that player 1 just put a piece
        match state.game_state.player_turn {
            PlayerTurn::Player1 => -1, // player 2 won
            PlayerTurn::Player2 => 1,  // player 1 won
        }
    }

    fn terminal(&self, state: &Game) -> bool {
        (0..BOARD_SIZE).any(|index| state.check_row_match(index) || state.check_column_match(index))
            || state.check_backward_slash_diagonal()
            || state.check_forward_slash_diagonal()
    }

    fn actions(&self, state: &Game) -> Vec<QuatroAction> {
        match state.game_state.stage {
            Stage::ChoosingPieceForOponent => state
                .get_pieces_left()
                .iter()
                .map(|piece| QuatroAction::Choose(*piece))
                .collect(),
            Stage::PlacingPieceGivenOponentChoice(_) => state
                .get_empty_places()
                .iter()
                .map(|position| QuatroAction::Put(*position))
                .collect(),
        }
    }

    fn result(&self, state: &Game, action: QuatroAction) -> Game {
        let mut new_state = state.clone();
        match action {
            QuatroAction::Choose(piece) => {
                new_state.choose(piece).unwrap();
                new_state
            }
            QuatroAction::Put(position) => {
                new_state.put(position).unwrap();
                new_state
            }
        }
    }
}
