#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn score_for_a_new_game_is_zero() {
        let game = Game::new();
        assert_eq!(game.score, 0);
    }
}

struct Game{
    score: i32
}

impl Game{
    fn new() -> Game {
        return Game{
            score: 0
        };
    }
}