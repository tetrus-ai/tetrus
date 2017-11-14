use ::engine::command::Command;
use ::engine::piece::Piece;
use ::objects::tetromino_generator::TetrominoGenerator;
use ::objects::up_next::UpNext;
use ::objects::well::Well;
use ::engine::piece_keeper::PieceKeeper;

pub struct Game{
    pub score: u32,
    pub well: Well,
    pub up_next: UpNext,
    pub current: Piece,
    piece_keeper: PieceKeeper
}

impl Game{
    pub fn new(generator: TetrominoGenerator) -> Game {
        let (generator, shape) = generator.next();
        let current = Piece::new(shape.unwrap());
        let up_next = UpNext::new(generator);
        Game {
            score: 0,
            well: Well::empty(),
            up_next: up_next,
            current,
            piece_keeper: PieceKeeper::default()
        }
    }

    pub fn tick(&self) -> Game {
        let current = self.current.drop_by_one();
        Game {
            score: self.score,
            well: self.well,
            up_next: self.up_next,
            current,
            piece_keeper: self.piece_keeper,
        }
    }

    pub fn issue_command(&self, command: Command) -> Game {
        let piece =  self.piece_keeper.execute_command(command, self.current);
        Game {
            score: self.score,
            well: self.well,
            up_next: self.up_next,
            current: piece,
            piece_keeper: self.piece_keeper,
        }
    }
}

#[cfg(test)]
mod new_game_should {
    use super::Game;
    use ::objects::tetromino_generator::TetrominoGenerator;

    #[test]
    fn have_a_score_of_zero() {
        let generator = TetrominoGenerator::default();
        let game = Game::new(generator);
        assert_eq!(game.score, 0);
    }
}

#[cfg(test)]
mod started_game_should {
    use super::Game;
    use ::engine::position::Position;
    use ::objects::shape::Shape;
    use ::objects::tetromino_generator::TetrominoGenerator;
    use ::rand::{StdRng, SeedableRng};

    #[test]
    fn have_a_tetromino_in_play() {
        let generator = TetrominoGenerator::new(StdRng::from_seed(&[1]));
        let game = Game::new(generator);
        assert_eq!(game.current.shape, Shape::l());
        assert_eq!(game.current.position, Position::new(5, 2));
    }
}

#[cfg(test)]
mod should {
    use super::Game;
    use ::engine::command::Command;
    use ::engine::position::Position;
    use ::objects::tetromino_generator::TetrominoGenerator;

    #[test]
    fn move_current_to_the_left_when_issued_a_move_left_command(){
        let generator = TetrominoGenerator::default();
        let game = Game::new(generator);
        let left_command = Command::MoveLeft;
        
        let game = game.issue_command(left_command);

        assert_eq!(game.current.position, Position::new(4, 2))
    }

    #[test]
    fn move_current_to_the_right_when_issued_a_move_right_command(){
        let generator = TetrominoGenerator::default();
        let game = Game::new(generator);
        let right_command = Command::MoveRight;
        
        let game = game.issue_command(right_command);

        assert_eq!(game.current.position, Position::new(6, 2))
    }
}