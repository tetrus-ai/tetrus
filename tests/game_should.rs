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
        let defaultTetromino = Tetromino::default();
        let generator = FakeGenerator::default();
        let game = Game::new(&generator);
        assert_eq!(game.up_next.first, defaultTetromino);
        assert_eq!(game.up_next.second, defaultTetromino);
    }

    #[derive(Default)]
    struct FakeGenerator{
    }

    impl GenerateTetromino for FakeGenerator{
        fn next(&self) -> Tetromino {
            Tetromino::default()
        }
    }
}