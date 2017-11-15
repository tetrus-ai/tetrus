extern crate tetrus;

#[cfg(test)]
mod banana {

    use tetrus::game::Game;
    use tetrus::pieces::TetrominoGenerator;
    use tetrus::movements::Command::*;

    #[test]
    fn stacking_two_pieces() {

        let generator = TetrominoGenerator::default();
        let game: Game = Game::new(generator);

        // assert that I can see 2 queued pieces and one active

        game.issue_command(MoveToLeftEdge);
        game.issue_command(DropToBottom);

        // assert that the queue has been updated

        game.issue_command(MoveToRightEdge);
        game.issue_command(DropToBottom);
        // assert that the queue has been updated
        // assert the final state of the well
    }

}
