extern crate tetrus;

mod game_should {
    use tetrus::game::Game;
    use tetrus::well::Well;
    #[test]
    fn have_score_zero_when_new () {
        let game = Game::new();
        assert_eq!(game.score, 0);
    }

    fn have_empty_well_when_new () {
        let game = Game::new();
        assert_eq!(game.well, Well::empty());
    }
}