use super::command::Command;
use super::piece::Piece;
use super::rules::out_of_bounds;

#[derive(Clone, Copy, Default)]
pub struct PieceKeeper{}

impl PieceKeeper{
    /* executes commands against a ruleset
       e.g. out_of_bounds::outside_of_left_boundary(&attempted_move)
    */
    pub fn execute_command(&self, command: Command, piece: Piece) -> Piece{
        let attempted_move = match command {
            Command::MoveLeft => piece.move_left(),
            Command::MoveRight => piece.move_right()
        };
        if out_of_bounds::outside_of_left_boundary(&attempted_move){
            piece
        } else {
            attempted_move
        }
    }
}