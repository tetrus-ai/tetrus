extern crate rand;
extern crate tetrus;

#[cfg(test)]
mod state_should {
    use tetrus::game::GameState;
    use tetrus::game::state::*;
    use tetrus::movements::Command::*;
    use tetrus::pieces::{Position, RandomTetrominoStream};
    use tetrus::pieces::shape::*;
    use rand::{SeedableRng, StdRng};

    #[test]
    fn have_score_zero_when_new() {
        let state = default();
        assert_eq!(state.score, 0);
    }

    #[test]
    fn populate_up_next() {
        let state = with_seed(&[3]);
        assert_eq!(state.next_pieces.first, S);
        assert_eq!(state.next_pieces.second, O);
    }

    #[test]
    fn place_current_tetromino_at_origin() {
        let state = with_seed(&[3]);
        assert_eq!(state.current_piece.shape, J);
        assert_eq!(state.current_piece.position, ORIGIN);
    }

    #[test]
    fn drop_current_piece_when_told_to_drop() {
        let state = default();
        let state = state.issue_command(Drop);
        assert_eq!(
            state.current_piece.position,
            Position::new(ORIGIN_X, ORIGIN_Y + MOVE_SPEED)
        )
    }

    #[test]
    fn move_current_piece_left_when_told_to_move_left() {
        let state = default();
        let state = state.issue_command(MoveLeft);
        assert_eq!(
            state.current_piece.position,
            Position::new(ORIGIN_X - MOVE_SPEED, ORIGIN_Y)
        )
    }

    #[test]
    fn move_current_piece_right_when_told_to_move_right() {
        let state = default();
        let state = state.issue_command(MoveRight);
        assert_eq!(
            state.current_piece.position,
            Position::new(ORIGIN_X + MOVE_SPEED, ORIGIN_Y)
        )
    }

    #[test]
    fn allow_a_piece_to_move_to_left_boundary() {
        let state = move_to_left_boundary(default());

        assert_eq!(
            state.current_piece.position,
            Position::new(BOUNDARY_LEFT, ORIGIN_Y)
        )
    }

    #[test]
    fn not_allow_a_piece_to_pass_left_boundary() {
        let state = move_to_left_boundary(default());
        let state = state.issue_command(MoveLeft);

        assert_eq!(
            state.current_piece.position,
            Position::new(BOUNDARY_LEFT, ORIGIN_Y)
        );
    }

    #[test]
    fn allow_a_piece_to_move_right_boundary() {
        let state = move_to_right_boundary(default());

        assert_eq!(
            state.current_piece.position,
            Position::new(BOUNDARY_RIGHT, ORIGIN_Y)
        );
    }

    #[test]
    fn not_allow_a_piece_to_pass_right_boundary() {
        let state = move_to_right_boundary(default());
        let state = state.issue_command(MoveRight);

        assert_eq!(
            state.current_piece.position,
            Position::new(BOUNDARY_RIGHT, ORIGIN_Y)
        );
    }

    #[test]
    fn allow_a_piece_to_drop_to_the_bottom() {
        let state = move_to_bottom(default());

        assert_eq!(
            state.current_piece.position,
            Position::new(ORIGIN_X, BOUNDARY_BOTTOM)
        );
    }

    fn default() -> GameState {
        let stream = RandomTetrominoStream::default();
        GameState::new(stream)
    }

    fn with_seed(seed: &[usize]) -> GameState {
        let rng = StdRng::from_seed(seed);
        let stream = RandomTetrominoStream::new(rng);
        GameState::new(stream)
    }

    fn move_to_left_boundary(state: GameState) -> GameState {
        match state.current_piece.position.x {
            BOUNDARY_LEFT => state,
            _ => move_to_left_boundary(state.issue_command(MoveLeft)),
        }
    }

    fn move_to_right_boundary(state: GameState) -> GameState {
        match state.current_piece.position.x {
            BOUNDARY_RIGHT => state,
            _ => move_to_right_boundary(state.issue_command(MoveRight)),
        }
    }

    fn move_to_bottom(state: GameState) -> GameState {
        match state.current_piece.position.y {
            BOUNDARY_BOTTOM => state,
            _ => move_to_bottom(state.issue_command(Drop)),
        }
    }
}
