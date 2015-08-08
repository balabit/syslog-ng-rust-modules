use state::State;
use context::BaseContext;

use self::message::Action as MessageAction;
use self::message::Command as MessageCommand;
pub use self::message::MessageActionHandler;

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

#[derive(Debug)]
pub enum ActionCommand {
    Message(MessageCommand)
}

pub struct ActionHandlers {
    message_handler: Box<MessageActionHandler>
}

impl ActionHandlers {
    pub fn new(message: Box<MessageActionHandler>) -> ActionHandlers {
        ActionHandlers {
            message_handler: message
        }
    }

    pub fn handle(&mut self, command: ActionCommand) {
        match command {
            ActionCommand::Message(message) => self.message_handler.handle(message)
        }
    }
}

pub mod message {
    use context::BaseContext;
    use state::State;
    use Message;
    use super::ActionCommand;

    #[derive(Debug)]
    pub struct Action;

    impl Action {
        pub fn execute(&self, _: &State, _: &BaseContext) -> ActionCommand {
            ActionCommand::Message(Command(Message::new("".to_string())))
        }
    }

    #[derive(Debug)]
    pub struct Command(Message);

    pub trait MessageActionHandler {
        fn handle(&mut self, command: Command);
    }
}
