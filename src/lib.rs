pub mod game {
    pub struct Game{
        pub score: i32
    }

    impl Game{
        pub fn new() -> Game {
            return Game{
                score: 0
            };
        }
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() {
            assert_eq!(2+2, 4);
        }
    }
}
