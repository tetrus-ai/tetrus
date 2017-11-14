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
        assert_eq!(game.current.position, Position::new(5, 2));
    }

    #[test]
    fn drop_current_piece_when_tick_finishes() {
        let game = default();
        let game = game.tick();
        assert_eq!(game.current.position, Position::new(5, 3));
    }

    #[test]
    fn move_current_piece_left_when_told_to_move_left() {
        let game = default();
        let left_command = Command::MoveLeft;
        let game = game.issue_command(left_command);
        assert_eq!(game.current.position, Position::new(4, 2))
    }

    #[test]
    fn move_current_piece_right_when_told_to_move_right() {
        let game = default();
        let right_command = Command::MoveRight;
        let game = game.issue_command(right_command);
        assert_eq!(game.current.position, Position::new(6, 2))
    }

    #[test]
    fn allow_a_piece_to_move_to_left_boundary() {
        let game = move_to_left_boundary(default());

        assert_eq!(game.current.position, Position::new(0, 2));
    }

    #[test]
    fn not_allow_a_piece_to_pass_left_boundary() {
        let game = move_to_left_boundary(default());
        let left_move = Command::MoveLeft;
        let game = game.issue_command(left_move);

        assert_eq!(game.current.position, Position::new(0, 2));
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
            0 => game,
            _ => move_to_left_boundary(game.issue_command(Command::MoveLeft))
        }
    }
}