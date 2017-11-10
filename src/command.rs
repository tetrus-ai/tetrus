pub struct Command{
    pub command_type: CommandType
}

impl Command{
    pub fn move_left() -> Command {
        Command {
            command_type: CommandType::MoveLeft
        }
    }

    pub fn move_right() -> Command {
        Command{
            command_type: CommandType::MoveRight
        }
    }
}

pub enum CommandType{
    MoveLeft,
    MoveRight
}
