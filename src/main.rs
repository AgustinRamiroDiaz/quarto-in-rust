mod coordinate;
use coordinate::Coordinate;
mod minimax;
mod piece;

mod board;

mod game;
use std::io::prelude::*;

use std::{
    collections::HashMap,
    fmt::{self, Debug},
};

fn main() -> Result<(), String> {
    let start_time = std::time::Instant::now();
    // let database_file_name = "state_to_value.json".to_string();
    let database_file_name = "state_to_value.bin".to_string();
    // let memory = read_from_json(&database_file_name);
    let memory = read_from_binary(&database_file_name);
    // let memory = HashMap::new();
    let finished_loading_time = std::time::Instant::now();
    println!(
        "Finished loading in {} seconds",
        finished_loading_time
            .duration_since(start_time)
            .as_secs_f32()
    );

    let mut game = game::Game::new();

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

    let mut qmm = QuatoMinimax::new(memory);
    // game.game_state.player_turn = game::Player::Player2;
    let initial_state = &game;
    let actions = qmm.actions(initial_state);

    let starting_inference_time = std::time::Instant::now();
    let actions_with_values = actions
        .iter()
        .map(|action| (action, qmm.min_value(initial_state)))
        .collect::<Vec<_>>();

    let finished_inference_time = std::time::Instant::now();
    println!(
        "Finished inference in {} seconds",
        finished_inference_time
            .duration_since(starting_inference_time)
            .as_secs_f32()
    );

    println!("BOARD\n{}\nBOARD", game.board);
    println!("{:?}", actions_with_values); // TODO: check why I'm always getting -1 :thinking

    let starting_saving_time = std::time::Instant::now();
    // write_to_json(&qmm.state_to_value, &database_file_name);
    write_to_binary(&qmm.state_to_value, &database_file_name);
    let finished_saving_time = std::time::Instant::now();
    println!(
        "Finished saving in {} seconds",
        finished_saving_time
            .duration_since(starting_saving_time)
            .as_secs_f32()
    );

    Ok(())
}

fn read_from_json(file_name: &String) -> HashMap<game::Game, i32> {
    let contents = std::fs::read_to_string(file_name).unwrap();
    let memory_string: HashMap<String, i32> = serde_json::from_str(&contents).unwrap();
    memory_string
        .into_iter()
        .map(|(key, value)| (serde_json::from_str(&key).unwrap(), value))
        .collect()
}

fn write_to_json(memory: &HashMap<game::Game, i32>, file_name: &String) {
    let serialized = serde_json::to_string(
        &memory
            .iter()
            .map(|(key, value)| (serde_json::to_string(key).unwrap(), *value))
            .collect::<HashMap<String, i32>>(),
    )
    .unwrap();

    let mut file = std::fs::File::create(file_name).unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
}

fn read_from_binary(file_name: &String) -> HashMap<game::Game, i32> {
    let contents = std::fs::read(file_name).unwrap();
    bincode::deserialize(&contents).unwrap()
}

fn write_to_binary(memory: &HashMap<game::Game, i32>, file_name: &String) {
    let serialized = bincode::serialize(memory).unwrap();
    let mut file = std::fs::File::create(file_name).unwrap();
    file.write_all(&serialized).unwrap();
}

const QUATRO: usize = 4;
const N_PROPERTIES: usize = 4;
const BOARD_SIZE: usize = 4;

impl Clone for board::Board<piece::Piece> {
    fn clone(&self) -> Self {
        board::Board { grid: self.grid }
    }
}

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

struct QuatoMinimax {
    state_to_value: HashMap<game::Game, i32>,
}

#[derive(Copy, Clone, Debug)]
enum QuatroAction {
    Choose(piece::Piece),
    Put(Coordinate),
}

impl QuatoMinimax {
    fn new(memory: HashMap<game::Game, i32>) -> QuatoMinimax {
        QuatoMinimax {
            state_to_value: memory,
        }
    }
}

// TODO: implementation for minimax is totally custom due to the actions not being the entire turn (2 actions per turn). When trying to implement the trait, I couln't figure out how to handle the imparity of the actions: a turn consists of "putting" a piece and then "choosing" one, but it's not aligned with how the game starts and ends by first "choosing" a piece and then "putting" it. This impacts in the implementation of the "actions" and "result" methods
impl QuatoMinimax {
    // We'll take into account the perspective of player 1 to calculate the utility
    // This function only makes sense for terminal states
    fn utility(&self, state: &game::Game) -> i32 {
        match state.game_state.result {
            game::GameResult::Draw => 0,
            game::GameResult::PlayerWon(player) => match player {
                game::Player::Player1 => 1,
                game::Player::Player2 => -1,
            },
            game::GameResult::InProgress => {
                panic!("Utility function called on non terminal state")
            }
        }
    }

    fn terminal(&self, state: &game::Game) -> bool {
        match state.game_state.result {
            game::GameResult::Draw => true,
            game::GameResult::PlayerWon(_) => true,
            game::GameResult::InProgress => false,
        }
    }

    fn actions(&self, state: &game::Game) -> Vec<QuatroAction> {
        match state.game_state.stage {
            game::Stage::ChoosingPieceForOponent => state
                .get_pieces_left()
                .iter()
                .map(|piece| QuatroAction::Choose(*piece))
                .collect(),
            game::Stage::PlacingPieceGivenOponentChoice(_) => state
                .get_empty_places()
                .iter()
                .map(|position| QuatroAction::Put(*position))
                .collect(),
        }
    }

    fn result(&self, state: &game::Game, action: QuatroAction) -> game::Game {
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

    fn min_value(&mut self, state: &game::Game) -> i32 {
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
    fn max_value(&mut self, state: &game::Game) -> i32 {
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
