use board_state;

fn main() {
    let one_piece: board_state::Piece = board_state::Piece{x: 3, y: 0, colour_bool: true, piece_type: String::from("Knight")};
    println!("{:?}", one_piece)
}
