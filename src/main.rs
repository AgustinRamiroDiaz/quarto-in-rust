mod coordinate;
use coordinate::Coordinate;

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

    // let qmm = QuatoMinimax::new();
    // let initial_state = &Game::new();
    // let actions = qmm.actions(initial_state);
    // let actions_with_values = actions
    //     .iter()
    //     .map(|action| (action, qmm.max_value(initial_state)))
    //     .collect::<Vec<_>>();

    // print!("{:#?}", actions_with_values);

    let pieces_with_coordinates = vec![
        // ([false, false, false, false], (0, 0)),
        ([false, false, false, true], (0, 1)),
        ([false, false, true, false], (0, 2)),
        ([false, true, false, false], (1, 0)),
        ([false, true, false, true], (1, 1)),
        ([false, true, true, false], (1, 2)),
        ([true, true, false, false], (2, 0)),
        ([true, true, false, true], (2, 1)),
        // ([true, true, true, false], (2, 2)),
    ];

    for (piece, (row, column)) in pieces_with_coordinates {
        game.choose(piece)?;
        game.put(Coordinate { row, column })?;
    }

    let qmm = QuatoMinimax::new();
    let initial_state = &game;
    let actions = qmm.actions(initial_state);
    let actions_with_values = actions
        .iter()
        .map(|action| (action, qmm.max_value(initial_state)))
        .collect::<Vec<_>>();

    println!("BOARD\n{}\nBOARD", game.board);
    print!("{:?}", actions_with_values); // TODO: check why I'm always getting -1 :thinking
    Ok(())
}

const QUATRO: usize = 4;
const N_PROPERTIES: usize = 4;
const BOARD_SIZE: usize = 4;

type Piece = [bool; N_PROPERTIES];

impl Clone for board::Board<Piece> {
    fn clone(&self) -> Self {
        board::Board { grid: self.grid }
    }
}

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

mod board;

impl<T: Debug + Copy> fmt::Display for board::Board<T> {
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
    player_turn: Player,
    stage: Stage,
    result: GameResult,
}

#[derive(Copy, Clone, Debug)]
enum Player {
    Player1,
    Player2,
}

#[derive(PartialEq, Clone, Debug)]
enum Stage {
    ChoosingPieceForOponent,
    PlacingPieceGivenOponentChoice(Piece),
}

#[derive(Clone, Debug)]
enum GameResult {
    InProgress,
    PlayerWon(Player),
    Draw,
}

#[derive(Clone, Debug)]
struct Game {
    board: board::Board<Piece>,
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
            board: board::Board::new(),
            game_state: GameState {
                player_turn: Player::Player1,
                stage: Stage::ChoosingPieceForOponent,
                result: GameResult::InProgress,
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
            .filter_map(|n| self.board.grid[n][BOARD_SIZE - n - 1])
            .collect::<Vec<Piece>>();

        forward_slash_diagonal.len() == QUATRO && check_match(forward_slash_diagonal)
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
                    Player::Player1 => Player::Player2,
                    Player::Player2 => Player::Player1,
                };
                Ok(())
            }
        }
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

    fn put(&mut self, position: Coordinate) -> Result<(), String> {
        // TODO: add player as parameter and check
        // TODO: reduce reading complexity
        match self.game_state.result {
            GameResult::Draw => Err("Game is over".to_string()),
            GameResult::PlayerWon(player) => Err(format!("Player {:?} won", player)),
            GameResult::InProgress => match self.game_state.stage {
                Stage::ChoosingPieceForOponent => {
                    Err("You can't place a piece right now".to_string())
                }
                Stage::PlacingPieceGivenOponentChoice(piece) => {
                    self.board.put(piece, position)?; // TODO: check if this changes the result

                    if self.pieces_left.is_empty() {
                        self.game_state.result = GameResult::Draw;
                    } else if self.check_if_won(position) {
                        self.game_state.result = GameResult::PlayerWon(self.game_state.player_turn);
                    } else {
                        self.game_state.stage = Stage::ChoosingPieceForOponent;
                        self.game_state.player_turn = match self.game_state.player_turn {
                            Player::Player1 => Player::Player2,
                            Player::Player2 => Player::Player1,
                        };
                    }

                    Ok(())
                }
            },
        }
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

    // Minimax will handle 3 possible values as result of the game:
    // 1 Player 1 wins
    // -1 Player 2 wins
    // 0 Draw
    // This information will be used to do optimizations
    fn min_value(&self, state: &State) -> i32 {
        if self.terminal(state) {
            return self.utility(state);
        }

        let mut v = i32::MAX;
        for action in self.actions(state) {
            v = v.min(self.max_value(&self.result(state, action)));
            if v == -1 {
                return v;
            }
        }

        v
    }
    fn max_value(&self, state: &State) -> i32 {
        if self.terminal(state) {
            return self.utility(state);
        }

        let mut v = i32::MIN;
        for action in self.actions(state) {
            v = v.max(self.min_value(&self.result(state, action)));
            if v == 1 {
                return v;
            }
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
        match state.game_state.result {
            GameResult::Draw => 0,
            GameResult::PlayerWon(player) => match player {
                Player::Player1 => 1,
                Player::Player2 => -1,
            },
            GameResult::InProgress => panic!("Utility function called on non terminal state"),
        }
    }

    fn terminal(&self, state: &Game) -> bool {
        match state.game_state.result {
            GameResult::Draw => true,
            GameResult::PlayerWon(_) => true,
            GameResult::InProgress => false,
        }
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
