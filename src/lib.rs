mod coordinate;
use coordinate::Coordinate;
mod minimax;
mod piece;
mod quatro_minimax;

mod board;

mod game;

use std::{
    collections::HashMap,
    fmt::{self, Debug},
};


pub fn themain() -> Result<(), String> {
    let start_time = std::time::Instant::now();
    // let database_file_name = "state_to_value.json".to_string();
    let database_file_name = "state_to_value.bin".to_string();
    // let memory = read_from_json(&database_file_name);
    // let memory = read_from_binary(&database_file_name);
    let memory = HashMap::new();
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
        game.choose(&piece)?;
        game.put(Coordinate { row, column })?;
    }

    let mut qmm = quatro_minimax::QuatoMinimax::new(memory);
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
    // write_to_binary(&qmm.state_to_value, &database_file_name);
    let finished_saving_time = std::time::Instant::now();
    println!(
        "Finished saving in {} seconds",
        finished_saving_time
            .duration_since(starting_saving_time)
            .as_secs_f32()
    );

    Ok(())
}

// fn read_from_json(file_name: &String) -> HashMap<game::Game, i32> {
//     let contents = std::fs::read_to_string(file_name).unwrap();
//     let memory_string: HashMap<String, i32> = serde_json::from_str(&contents).unwrap();
//     memory_string
//         .into_iter()
//         .map(|(key, value)| (serde_json::from_str(&key).unwrap(), value))
//         .collect()
// }

// fn write_to_json(memory: &HashMap<game::Game, i32>, file_name: &String) {
//     let serialized = serde_json::to_string(
//         &memory
//             .iter()
//             .map(|(key, value)| (serde_json::to_string(key).unwrap(), *value))
//             .collect::<HashMap<String, i32>>(),
//     )
//     .unwrap();

//     let mut file = std::fs::File::create(file_name).unwrap();
//     file.write_all(serialized.as_bytes()).unwrap();
// }

// fn read_from_binary(file_name: &String) -> HashMap<game::Game, i32> {
//     let contents = std::fs::read(file_name).unwrap();
//     bincode::deserialize(&contents).unwrap()
// }

// fn write_to_binary(memory: &HashMap<game::Game, i32>, file_name: &String) {
//     let serialized = bincode::serialize(memory).unwrap();
//     let mut file = std::fs::File::create(file_name).unwrap();
//     file.write_all(&serialized).unwrap();
// }

const QUATRO: usize = 4;
const N_PROPERTIES: usize = 4;
const BOARD_SIZE: usize = 4;

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
