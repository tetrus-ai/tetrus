extern crate tetrus;

#[cfg(test)]
mod feature {
    use tetrus::game::Game;
    use tetrus::game::PlayAreaSize;
    use tetrus::pieces::RandomTetrominoStream;
    use tetrus::pieces::RandomTetrominoServer;
    use tetrus::movements::Command::*;
    use tetrus::movements::DefaultMotionController;
    use tetrus::rules::RuleSet;

    #[test]
    fn stacking_two_pieces() {
        let buffer = RandomTetrominoServer::new(RandomTetrominoStream::default());
        let narrow_play_area = PlayAreaSize::with_width_and_height(3, 6);
        let ruleset = RuleSet::with_play_area_size(narrow_play_area);
        let motion_controller = DefaultMotionController::new(ruleset);
        let game = Game::new(buffer, &motion_controller);

        let next_pieces = game.next_pieces;

        let game = game.issue_command(MoveToLeftEdge);

        let game = game.issue_command(DropToBottom);

        assert_eq!(next_pieces.first, game.current.shape);
        assert_eq!(next_pieces.second, game.next_pieces.first);

        let next_pieces = game.next_pieces;

        let game = game.issue_command(MoveToRightEdge);
        let game = game.issue_command(DropToBottom);

        assert_eq!(next_pieces.first, game.current.shape);
        assert_eq!(next_pieces.second, game.next_pieces.first);

        // TODO: assert the final state of the well
    }
}
