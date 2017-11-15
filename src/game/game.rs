use movements::Command;
use super::Game;
use super::PlayAreaSize;

impl Game {
    pub fn default_ruleset(size: PlayAreaSize) -> Self {
        Game {}
    }

    pub fn issue_command(&self, command: Command) {}
}
