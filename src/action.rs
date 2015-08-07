use state::State;
use context::BaseContext;

use self::message::Action as MessageAction;
use self::message::Command as MessageCommand;

#[derive(Debug)]
pub enum Action {
    Message(MessageAction)
}

impl Action {
    pub fn execute(&self, state: &State, context: &BaseContext) -> ActionCommand {
        match *self {
            Action::Message(ref action) => action.execute(state, context)
        }
    }
}

pub enum ActionCommand {
    Message(MessageCommand)
}

mod message {
    use context::BaseContext;
    use state::State;
    use Message;
    use super::ActionCommand;

    #[derive(Debug)]
    pub struct Action;

    impl Action {
        pub fn execute(&self, state: &State, context: &BaseContext) -> ActionCommand {
            ActionCommand::Message(Command(Message::new("".to_string())))
        }
    }

    pub struct Command(Message);
}
