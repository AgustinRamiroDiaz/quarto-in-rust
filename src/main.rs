mod board;
mod coordinate;
mod game;
mod minimax;
mod piece;

use coordinate::Coordinate;
use minimax::Minimax;

use std::fmt::{self, Debug};

fn main() -> Result<(), String> {
    let mut game = game::Game::new();
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
    game.choose([true, true, true, false])?;

    let qmm = QuatoMinimax::new();
    print!("{:#?}", game.game_state);
    let initial_state = &game;
    let actions = qmm.actions(initial_state);
    let actions_with_values = actions
        .iter()
        .map(|action| (action, qmm.max_value(initial_state)))
        .collect::<Vec<_>>();

    println!("BOARD\n{}\nBOARD", game.board);
    print!("{:?}", actions_with_values); // TODO: check why I'm always getting -1 🤔
    Ok(())
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

struct QuatoMinimax {}

#[derive(Copy, Clone, Debug)]
struct QuatroAction {
    put: Coordinate,
    choose: piece::Piece,
}

impl QuatoMinimax {
    fn new() -> QuatoMinimax {
        QuatoMinimax {}
    }
}

impl minimax::Minimax<game::Game, QuatroAction> for QuatoMinimax {
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
            game::Stage::ChoosingPieceForOponent => panic!("Shouldn't be called in this stage, because actions are considered as putting + choosing (everything you can do in your turn). This is an implementation detail due to how the Minimax trais is defined"), // TODO: abstract the turns from the minimax states
            game::Stage::PlacingPieceGivenOponentChoice(_) => 
            {
                let places_left = state.get_empty_places();
                let pieces_left = state.get_pieces_left();
                let mut actions = Vec::new();
                for place in places_left {
                    for piece in &pieces_left {
                        actions.push(QuatroAction {
                            put: place,
                            choose: *piece,
                        });
                    }
                }
                actions
            }
        }
    }


    fn result(&self, state: &game::Game, action: QuatroAction) -> game::Game {
        let mut new_state = state.clone();
        new_state.put(action.put).unwrap(); // TODO: handle errors
        new_state.choose(action.choose).unwrap();
        new_state
    }
}
