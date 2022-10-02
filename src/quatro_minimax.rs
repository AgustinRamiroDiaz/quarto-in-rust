use crate::coordinate::Coordinate;
use crate::game;
use crate::game::Game;
use crate::game::GameResult;
use crate::piece;

use std::collections::HashMap;

pub(crate) struct QuatoMinimax<'a> {
    pub(crate) state_to_value: HashMap<Game<'a>, i32>,
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum QuatroAction<'a> {
    Choose(&'a piece::Piece),
    Put(Coordinate),
}

impl<'a> QuatoMinimax<'a> {
    pub(crate) fn new(memory: HashMap<Game, i32>) -> QuatoMinimax {
        QuatoMinimax {
            state_to_value: memory,
        }
    }
}

// TODO: implementation for minimax is totally custom due to the actions not being the entire turn (2 actions per turn). When trying to implement the trait, I couln't figure out how to handle the imparity of the actions: a turn consists of "putting" a piece and then "choosing" one, but it's not aligned with how the game starts and ends by first "choosing" a piece and then "putting" it. This impacts in the implementation of the "actions" and "result" methods
impl<'a> QuatoMinimax<'a> {
    // We'll take into account the perspective of player 1 to calculate the utility
    // This function only makes sense for terminal states
    pub(crate) fn utility(&self, state: &Game) -> i32 {
        match state.game_state.result {
            GameResult::Draw => 0,
            GameResult::PlayerWon(player) => match player {
                game::Player::Player1 => 1,
                game::Player::Player2 => -1,
            },
            GameResult::InProgress => {
                panic!("Utility function called on non terminal state")
            }
        }
    }

    pub(crate) fn terminal(&self, state: &Game) -> bool {
        match state.game_state.result {
            GameResult::Draw => true,
            GameResult::PlayerWon(_) => true,
            GameResult::InProgress => false,
        }
    }

    pub(crate) fn actions(&self, state: &'a Game) -> Vec<QuatroAction> {
        match state.game_state.stage {
            game::Stage::ChoosingPieceForOponent => state
                .get_pieces_left()
                .iter()
                .map(|piece| QuatroAction::Choose(piece))
                .collect(),
            game::Stage::PlacingPieceGivenOponentChoice(_) => state
                .get_empty_places()
                .iter()
                .map(|position| QuatroAction::Put(*position))
                .collect(),
        }
    }

    pub(crate) fn result(&self, state: &'a Game, action: QuatroAction) -> Game {
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

    pub(crate) fn min_value(&'a mut self, state: &'a Game) -> i32 {
        if state.game_state.player_turn != game::Player::Player2 {
            panic!("Min value called on a state where it's not player 2 turn");
        }

        match self.state_to_value.get(state) {
            Some(value) => return *value,
            None => (),
        }

        if self.terminal(state) {
            return self.utility(state);
        }

        let m_value = match state.game_state.stage {
            game::Stage::PlacingPieceGivenOponentChoice(_) => QuatoMinimax::min_value,
            game::Stage::ChoosingPieceForOponent => QuatoMinimax::max_value,
        };

        let mut v = i32::MAX;
        for action in self.actions(state) {
            v = v.min(m_value(self, &self.result(state, action)));
            if v == -1 {
                break;
            }
        }

        self.state_to_value.insert(state.clone(), v);
        v
    }
    pub(crate) fn max_value(&'a mut self, state: &'a Game) -> i32 {
        if state.game_state.player_turn != game::Player::Player1 {
            panic!("Max value called on a state where it's not player 1 turn");
        }

        match self.state_to_value.get(state) {
            Some(value) => return *value,
            None => (),
        }

        if self.terminal(state) {
            return self.utility(state);
        }

        let m_value = match state.game_state.stage {
            game::Stage::PlacingPieceGivenOponentChoice(_) => QuatoMinimax::max_value,
            game::Stage::ChoosingPieceForOponent => QuatoMinimax::min_value,
        };

        let mut v = i32::MIN;
        for action in self.actions(state) {
            v = v.max(m_value(self, &self.result(state, action)));
            if v == 1 {
                break;
            }
        }

        self.state_to_value.insert(state.clone(), v);

        v
    }
}
