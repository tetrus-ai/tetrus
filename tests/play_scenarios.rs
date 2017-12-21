extern crate tetrus;

#[cfg(test)]
mod feature {
    use tetrus::game::Game;
    use tetrus::game::PlayAreaSize;
    use tetrus::pieces::RandomTetrominoStream;
    use tetrus::pieces::RandomTetrominoBuffer;
    use tetrus::movements::Command::*;
    use tetrus::movements::PieceKeeper;

    #[test]
    fn stacking_two_pieces() {
        let buffer = RandomTetrominoBuffer::new(RandomTetrominoStream::default());
        let narrow_play_area = PlayAreaSize::with_width_and_height(3, 6);
        let motion_controller = PieceKeeper::default();
        let mut game = Game::default_ruleset(narrow_play_area, buffer, &motion_controller);

        let next_pieces = game.next_pieces;

        game.issue_command(MoveToLeftEdge);

        game.issue_command(DropToBottom);

        assert_eq!(next_pieces.first, game.current.shape);
        assert_eq!(next_pieces.second, game.next_pieces.first);

        let next_pieces = game.next_pieces;

        game.issue_command(MoveToRightEdge);
        game.issue_command(DropToBottom);

        assert_eq!(next_pieces.first, game.current.shape);
        assert_eq!(next_pieces.second, game.next_pieces.first);

        // TODO: assert the final state of the well
    }
}
