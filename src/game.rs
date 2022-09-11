use super::QUATRO;
use crate::board::Board;
use crate::Coordinate;

use super::piece::check_match;

use super::BOARD_SIZE;

use super::N_PROPERTIES;

use super::piece::Piece;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct GameState {
    pub(crate) player_turn: Player,
    pub(crate) stage: Stage,
    pub(crate) result: GameResult,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Player {
    Player1,
    Player2,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Stage {
    ChoosingPieceForOponent,
    PlacingPieceGivenOponentChoice(Piece),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum GameResult {
    InProgress,
    PlayerWon(Player),
    Draw,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Game {
    pub(crate) board: Board<Piece>,
    pub(crate) game_state: GameState,
    pub(crate) pieces_left: HashSet<Piece>,
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board && self.game_state == other.game_state // we don't care about pieces left, it does not affect the game state (kindof)
    }
}

impl Eq for Game {}

impl Hash for Game {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.board.hash(state);
        self.game_state.hash(state);
    }
}

impl Game {
    pub(crate) fn new() -> Game {
        let mut pieces = HashSet::<Piece>::new();
        for piece in all_possible_pieces(N_PROPERTIES) {
            pieces.insert(piece.try_into().unwrap()); // TODO: may panic
        }

        Game {
            board: Board::new(),
            game_state: GameState {
                player_turn: Player::Player1,
                stage: Stage::ChoosingPieceForOponent,
                result: GameResult::InProgress,
            },
            pieces_left: pieces,
        }
    }

    pub(crate) fn check_row_match(&self, row: usize) -> bool {
        let row_items = self.board.grid[row]
            .into_iter()
            .flatten()
            .collect::<Vec<Piece>>();
        row_items.len() == BOARD_SIZE && check_match(row_items)
    }

    pub(crate) fn check_column_match(&self, column: usize) -> bool {
        let column_items = self
            .board
            .grid
            .iter()
            .filter_map(|row| row[column])
            .collect::<Vec<Piece>>();

        column_items.len() == QUATRO && check_match(column_items)
    }

    pub(crate) fn check_backward_slash_diagonal(&self) -> bool {
        let backward_slash_diagonal = (0..BOARD_SIZE)
            .filter_map(|n| self.board.grid[n][n])
            .collect::<Vec<Piece>>();

        backward_slash_diagonal.len() == QUATRO && check_match(backward_slash_diagonal)
    }

    pub(crate) fn check_forward_slash_diagonal(&self) -> bool {
        let forward_slash_diagonal = (0..BOARD_SIZE)
            .filter_map(|n| self.board.grid[n][BOARD_SIZE - n - 1])
            .collect::<Vec<Piece>>();

        forward_slash_diagonal.len() == QUATRO && check_match(forward_slash_diagonal)
    }

    pub(crate) fn get_pieces_left(&self) -> Vec<Piece> {
        self.pieces_left.iter().cloned().collect()
    }

    pub(crate) fn get_empty_places(&self) -> Vec<Coordinate> {
        self.board.empty_spaces()
    }

    pub(crate) fn choose(&mut self, piece: Piece) -> Result<(), String> {
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

    pub(crate) fn check_if_won(&self, position: Coordinate) -> bool {
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

    pub(crate) fn put(&mut self, position: Coordinate) -> Result<(), String> {
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
                    }

                    Ok(())
                }
            },
        }
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
