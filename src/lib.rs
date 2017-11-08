//this is super weird that I have to do this
//surely the compiler is aware that these modules exist on the STD namespace?
mod square;
pub mod well;

pub mod game {
    use super::well::*;

    pub struct Game{
        pub score: i32,
        pub well: Well
    }

    impl Game{
        pub fn new() -> Game {
            return Game{
                score: 0,
                well: Well::empty()
            };
        }
    }

    #[cfg(test)]
    mod constructor_should {
        use super::Game;

        #[test]
        fn set_score_to_zero() {
            let game = Game::new();
            assert_eq!(game.score, 0);
        }
    }
}
