fn main() {
    println!("{:#?}", empty_board());
}

type Piece = [bool; 4];

type Board<T> = [[Option<T>; 4]; 4];

struct Game {
    board: Board<Piece>,
}

fn empty_row() -> [Option<Piece>; 4] {
    [None; 4]
}

fn empty_board() -> Board<Piece> {
    [empty_row(); 4]
}



// impl Game {
//     fn new() -> Game {
//         Game {
//             board: emptyBoard(),
//         }
//     }
// }
