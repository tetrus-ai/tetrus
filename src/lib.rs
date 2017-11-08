// this is super weird that I have to do this
// surely the compiler is aware that these modules exist on the STD namespace?
mod square;
mod tetromino;
mod up_next;
pub mod well;

pub mod game {
    use super::well::Well;
    use super::tetromino::Tetromino;
    use super::up_next::UpNext;

    #[derive(Default)]
    pub struct Game{
        pub score: u32,
        pub well: Well,
        pub up_next: UpNext
    }

    impl Game{
        pub fn new() -> Game {
            Game {
                score: 0,
                well: Well::empty(),
                up_next: UpNext::default()
            }
        }
    }

    #[cfg(test)]
    mod constructor_should {
        use super::Game;

        #[test]
        fn set_score_to_zero() {
            let game = Game::default();
            assert_eq!(game.score, 0);
        }
    }
}
