use state::State;
use context::BaseContext;

pub use self::handlers::ActionHandlers;
pub use self::message::{MessageAction, MessageActionHandler, MessageCommand};

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

pub mod handlers {
    use super::{ActionCommand, MessageActionHandler};

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
}

pub mod message {
    use context::BaseContext;
    use state::State;
    use Message;
    use super::ActionCommand;

    #[derive(Debug)]
    pub struct MessageAction;

    impl MessageAction {
        pub fn execute(&self, _: &State, _: &BaseContext) -> ActionCommand {
            ActionCommand::Message(MessageCommand(Message::new("".to_string())))
        }
    }

    #[derive(Debug)]
    pub struct MessageCommand(Message);

    pub trait MessageActionHandler {
        fn handle(&mut self, command: MessageCommand);
    }
}
