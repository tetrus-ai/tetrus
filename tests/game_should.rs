extern crate rand;
extern crate tetrus;

mod game_should {
    use tetrus::game::Game;
    use tetrus::engine::command::Command;
    use tetrus::engine::position::Position;
    use tetrus::objects::shape::Shape;
    use tetrus::objects::tetromino_generator::TetrominoGenerator;
    use tetrus::objects::well::Well;
    use rand::{SeedableRng, StdRng};

    const ORIGIN: Position = Position{ x:ORIGIN_X, y:ORIGIN_Y };
    const ORIGIN_X: i8 = 0;
    const ORIGIN_Y: i8 = 0;

    const BOUNDARY_LEFT: i8 = -4;
    const BOUNDARY_RIGHT: i8 = 4;
    const BOUNDARY_BOTTOM: i8 = 20;

    #[test]
    fn have_score_zero_when_new() {
        let game = default();
        assert_eq!(game.score, 0);
    }

    #[test]
    fn have_empty_well_when_new() {
        let game = default();
        assert_eq!(game.well, Well::empty());
    }

    #[test]
    fn populate_up_next() {
        let game = with_seed(&[2]);
        assert_eq!(game.up_next.first, Shape::z());
        assert_eq!(game.up_next.second, Shape::o());
    }

    #[test]
    fn place_current_tetromino_at_5_2() {
        let game = default();
        assert_eq!(game.current.shape, Shape::i());
        assert_eq!(game.current.position, ORIGIN);
    }

    #[test]
    fn drop_current_piece_when_tick_finishes() {
        let game = default();
        let game = game.tick();
        assert_eq!(game.current.position, Position::new(ORIGIN_X, ORIGIN_Y + 1));
    }

    #[test]
    fn move_current_piece_left_when_told_to_move_left() {
        let game = default();
        let left_command = Command::MoveLeft;
        let game = game.issue_command(left_command);
        assert_eq!(game.current.position, Position::new(ORIGIN_X - 1, ORIGIN_Y))
    }

    #[test]
    fn move_current_piece_right_when_told_to_move_right() {
        let game = default();
        let right_command = Command::MoveRight;
        let game = game.issue_command(right_command);
        assert_eq!(game.current.position, Position::new(ORIGIN_X + 1, ORIGIN_Y))
    }

    #[test]
    fn allow_a_piece_to_move_to_left_boundary() {
        let game = move_to_left_boundary(default());

        assert_eq!(game.current.position, Position::new(BOUNDARY_LEFT, ORIGIN_Y));
    }

    #[test]
    fn not_allow_a_piece_to_pass_left_boundary() {
        let game = move_to_left_boundary(default());
        let game = game.issue_command(Command::MoveLeft);

        assert_eq!(game.current.position, Position::new(BOUNDARY_LEFT, ORIGIN_Y));
    }

    #[test]
    fn allow_a_piece_to_move_right_boundary() {
        let game = move_to_right_boundary(default());

        assert_eq!(game.current.position, Position::new(BOUNDARY_RIGHT, ORIGIN_Y));
    }

    #[test]
    fn not_allow_a_piece_to_pass_right_boundary() {
        let game = move_to_right_boundary(default());
        let game = game.issue_command(Command::MoveRight);

        assert_eq!(game.current.position, Position::new(BOUNDARY_RIGHT, ORIGIN_Y));
    }

    #[test]
    fn allow_a_piece_to_drop_to_the_bottom() {
        let game = move_to_bottom(default());

        assert_eq!(game.current.position, Position::new(ORIGIN_X, BOUNDARY_BOTTOM));
    }

    fn default() -> Game{
        let generator = TetrominoGenerator::default();
        Game::new(generator)
    }

    fn with_seed(seed: &[usize]) -> Game{
        let rng = StdRng::from_seed(seed);
        let generator = TetrominoGenerator::new(rng);
        Game::new(generator)
    }

    fn move_to_left_boundary(game: Game) -> Game{
        match game.current.position.x{
            BOUNDARY_LEFT => game,
            _ => move_to_left_boundary(game.issue_command(Command::MoveLeft))
        }
    }

    fn move_to_right_boundary(game: Game ) -> Game{
        match game.current.position.x{
            BOUNDARY_RIGHT => game,
            _ => move_to_right_boundary(game.issue_command(Command::MoveRight))
        }
    }

    fn move_to_bottom(game: Game ) -> Game{
        match game.current.position.y{
            BOUNDARY_BOTTOM => game,
            _ => move_to_bottom(game.issue_command(Command::Drop))
        }
    }
}