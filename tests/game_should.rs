extern crate tetrus;
extern crate rand;

mod game_should {
    use tetrus::game::Game;
    use tetrus::well::Well;
    use tetrus::tetromino::Tetromino;
    use tetrus::tetromino_generator::{TetrominoGenerator};
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
        assert_eq!(game.tetrominos.current, Tetromino::I);
        assert_eq!(game.tetrominos.first, Tetromino::Z);
        assert_eq!(game.tetrominos.second, Tetromino::O);
    }

    #[test]
    fn place_current_tetromino_at_5_2() {}
}