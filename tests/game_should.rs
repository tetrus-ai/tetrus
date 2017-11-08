extern crate tetrus;

mod game_should {
    use tetrus::game::Game;
    use tetrus::well::Well;
    use tetrus::tetromino::Tetromino;
    use tetrus::tetromino_generator::GenerateTetromino;

    #[test]
    fn have_score_zero_when_new () {
        let generator = FakeGenerator::default();
        let game = Game::new(&generator);
        assert_eq!(game.score, 0);
    }

    #[test]
    fn have_empty_well_when_new () {
        let generator = FakeGenerator::default();
        let game = Game::new(&generator);
        assert_eq!(game.well, Well::empty());
    }

    #[test]
    fn populate_up_next () {
        let default_tetromino = Tetromino::default();
        let generator = FakeGenerator::new(default_tetromino);
        let game = Game::new(&generator);
        assert_eq!(game.up_next.first, default_tetromino);
        assert_eq!(game.up_next.second, default_tetromino);
    }

    #[test]
    fn move_first_to_in_play_when_game_starts () {
        let default_tetromino = Tetromino::default();
        let generator = FakeGenerator::new(default_tetromino);
        let mut game = Game::new(&generator);

        game.start();

        assert_eq!(game.in_play, Some(default_tetromino));
    }

    #[derive(Default)]
    struct FakeGenerator{
        tetromino_to_generate: Tetromino
    }

    impl FakeGenerator{
        fn new(tetromino: Tetromino) -> FakeGenerator{
            FakeGenerator{
                tetromino_to_generate: tetromino
            }
        }
    }

    impl GenerateTetromino for FakeGenerator{
        fn next(&self) -> Tetromino {
            self.tetromino_to_generate
        }
    }
}