extern crate tetrus;

mod tests {
    use tetrus::game;
    #[test]
    fn have_score_zero_when_new () {
        let game = game::Game::new();
        assert_eq!(game.score, 0);
    }
}