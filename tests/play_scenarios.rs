extern crate tetrus;

#[cfg(test)]
mod banana {
    use tetrus::game::Game;
    use tetrus::game::PlayAreaSize;
    use tetrus::pieces::RandomTetrominoStream;
    use tetrus::movements::Command::*;

    #[test]
    fn stacking_two_pieces() {
        let game = Game::default_ruleset(PlayAreaSize::with_width_and_height(3, 6));

        let next_pieces = game.next_pieces;

        game.issue_command(MoveToLeftEdge);

        game.issue_command(DropToBottom);

        assert_ne!(next_pieces, game.next_pieces);

        game.issue_command(MoveToRightEdge);
        game.issue_command(DropToBottom);

        // TODO: assert that the queue has been updated

        // TODO: assert the final state of the well
    }
}
