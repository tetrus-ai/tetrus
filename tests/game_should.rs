extern crate tetrus;
extern crate rand;

mod game_should {
    use tetrus::game::Game;
    use tetrus::well::Well;
    use tetrus::tetromino::Tetromino;
    use tetrus::tetromino_generator::{TetrominoGenerator, GenerateTetromino};
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
        let rng = StdRng::from_seed(&[1]);
        let mut generator = TetrominoGenerator::new(rng);
        let game = Game::new(&mut generator);
        assert_eq!(game.up_next.first, Tetromino::O);
        assert_eq!(game.up_next.second, Tetromino::L);
    }

    #[test]
    fn move_first_to_in_play_when_game_starts () {
        let rng = StdRng::from_seed(&[2]);
        let mut generator = TetrominoGenerator::new(rng);
        let mut game = Game::new(&mut generator);

        let game = game.start();

        assert_eq!(game.in_play, Tetromino::Z);
        assert_eq!(game.up_next.first, Tetromino::O);
        assert_eq!(game.up_next.second, Tetromino::I);
    }
}