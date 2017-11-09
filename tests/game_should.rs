extern crate tetrus;
extern crate rand;

mod game_should {
    use tetrus::game::Game;
    use tetrus::position::Position;
    use tetrus::tetromino::Tetromino;
    use tetrus::tetromino_generator::{TetrominoGenerator};
    use tetrus::well::Well;
    use ::rand::{StdRng, SeedableRng};

    #[test]
    fn have_score_zero_when_new () {
        let mut generator = TetrominoGenerator::default();
        let game = Game::new(&mut generator);
        assert_eq!(game.score, 0);
    }

    #[test]
    fn have_empty_well_when_new () {
        let mut generator = TetrominoGenerator::default();
        let game = Game::new(&mut generator);
        assert_eq!(game.well, Well::empty());
    }

    #[test]
    fn populate_up_next () {
        let rng = StdRng::from_seed(&[2]);
        let mut generator = TetrominoGenerator::new(rng);
        let game = Game::new(&mut generator);
        assert_eq!(game.up_next.first, Tetromino::z());
        assert_eq!(game.up_next.second, Tetromino::o());
    }

    #[test]
    fn place_current_tetromino_at_5_2() {
        let mut generator = TetrominoGenerator::default();
        let game = Game::new(&mut generator);
        assert_eq!(game.current.tetromino, Tetromino::i());
        assert_eq!(game.current.position, Position::new(5, 2));
    }
}