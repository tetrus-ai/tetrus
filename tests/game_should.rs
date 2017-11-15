extern crate rand;
extern crate tetrus;

#[cfg(test)]
mod game_should {
    use tetrus::game::*;
    use tetrus::movements::Command::*;
    use tetrus::pieces::{Position, Shape, TetrominoGenerator};
    use tetrus::pieces::shape::*;
    use rand::{SeedableRng, StdRng};

    #[test]
    fn have_score_zero_when_new() {
        let game = default();
        assert_eq!(game.score, 0);
    }

    #[test]
    fn populate_up_next() {
        let game = with_seed(&[2]);
        assert_eq!(game.up_next.first, Z);
        assert_eq!(game.up_next.second, O);
    }

    #[test]
    fn place_current_tetromino_at_origin() {
        let game = default();
        assert_eq!(game.current_piece.shape, I);
        assert_eq!(game.current_piece.position, ORIGIN);
    }

    #[test]
    fn drop_current_piece_when_told_to_drop() {
        let game = default();
        let game = game.issue_command(Drop);
        assert_eq!(
            game.current_piece.position,
            Position::new(ORIGIN_X, ORIGIN_Y + MOVE_SPEED)
        )
    }

    #[test]
    fn move_current_piece_left_when_told_to_move_left() {
        let game = default();
        let game = game.issue_command(MoveLeft);
        assert_eq!(
            game.current_piece.position,
            Position::new(ORIGIN_X - MOVE_SPEED, ORIGIN_Y)
        )
    }

    #[test]
    fn move_current_piece_right_when_told_to_move_right() {
        let game = default();
        let game = game.issue_command(MoveRight);
        assert_eq!(
            game.current_piece.position,
            Position::new(ORIGIN_X + MOVE_SPEED, ORIGIN_Y)
        )
    }

    #[test]
    fn allow_a_piece_to_move_to_left_boundary() {
        let game = move_to_left_boundary(default());

        assert_eq!(
            game.current_piece.position,
            Position::new(BOUNDARY_LEFT, ORIGIN_Y)
        )
    }

    #[test]
    fn not_allow_a_piece_to_pass_left_boundary() {
        let game = move_to_left_boundary(default());
        let game = game.issue_command(MoveLeft);

        assert_eq!(
            game.current_piece.position,
            Position::new(BOUNDARY_LEFT, ORIGIN_Y)
        );
    }

    #[test]
    fn allow_a_piece_to_move_right_boundary() {
        let game = move_to_right_boundary(default());

        assert_eq!(
            game.current_piece.position,
            Position::new(BOUNDARY_RIGHT, ORIGIN_Y)
        );
    }

    #[test]
    fn not_allow_a_piece_to_pass_right_boundary() {
        let game = move_to_right_boundary(default());
        let game = game.issue_command(MoveRight);

        assert_eq!(
            game.current_piece.position,
            Position::new(BOUNDARY_RIGHT, ORIGIN_Y)
        );
    }

    #[test]
    fn allow_a_piece_to_drop_to_the_bottom() {
        let game = move_to_bottom(default());

        assert_eq!(
            game.current_piece.position,
            Position::new(ORIGIN_X, BOUNDARY_BOTTOM)
        );
    }

    fn default() -> Game {
        let generator = TetrominoGenerator::default();
        Game::new(generator)
    }

    fn with_seed(seed: &[usize]) -> Game {
        let rng = StdRng::from_seed(seed);
        let generator = TetrominoGenerator::new(rng);
        Game::new(generator)
    }

    fn move_to_left_boundary(game: Game) -> Game {
        match game.current_piece.position.x {
            BOUNDARY_LEFT => game,
            _ => move_to_left_boundary(game.issue_command(MoveLeft)),
        }
    }

    fn move_to_right_boundary(game: Game) -> Game {
        match game.current_piece.position.x {
            BOUNDARY_RIGHT => game,
            _ => move_to_right_boundary(game.issue_command(MoveRight)),
        }
    }

    fn move_to_bottom(game: Game) -> Game {
        match game.current_piece.position.y {
            BOUNDARY_BOTTOM => game,
            _ => move_to_bottom(game.issue_command(Drop)),
        }
    }
}
