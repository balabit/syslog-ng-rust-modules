use state::State;
use context::BaseContext;

pub use self::handlers::ActionHandlers;

#[derive(Clone, Debug)]
pub enum Action {
    Message(self::message::Action)
}

impl Action {
    pub fn execute(&self, state: &State, context: &BaseContext) -> ExecResult {
        let result = match *self {
            Action::Message(ref action) => action.execute(state, context)
        };

        ExecResult::from(result)
    }
}

#[derive(Debug)]
pub enum ExecResult {
    Message(self::message::ExecResult)
}

pub mod handlers {
    use super::ExecResult;
    use super::message;

    pub struct ActionHandlers {
        message_handler: Box<message::ActionHandler>
    }

    impl ActionHandlers {
        pub fn new(message: Box<message::ActionHandler>) -> ActionHandlers {
            ActionHandlers {
                message_handler: message
            }
        }

        pub fn handle(&mut self, command: ExecResult) {
            match command {
                ExecResult::Message(message) => self.message_handler.handle(message)
            }
        }
    }
}

pub mod message {
    use context::BaseContext;
    use state::State;
    use Message;

    #[derive(Debug)]
    pub struct ExecResult(Message);

    impl From<ExecResult> for super::ExecResult {
        fn from(result: ExecResult) -> super::ExecResult {
            super::ExecResult::Message(result)
        }
    }

    #[derive(Clone, Debug)]
    pub struct Action;

    impl Action {
        pub fn execute(&self, _: &State, _: &BaseContext) -> ExecResult {
            ExecResult(Message::new("".to_string()))
        }
    }

    pub trait ActionHandler {
        fn handle(&mut self, command: ExecResult);
    }
}
