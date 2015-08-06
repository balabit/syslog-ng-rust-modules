use state::State;
use context::Context;

use self::message::Action as MessageAction;

#[derive(Debug)]
pub enum Action {
    Message(MessageAction)
}

impl Action {
    pub fn execute(&self, state: &State, context: &Context) {
        match *self {
            Action::Message(ref action) => action.execute(state, context)
        }
    }
}

mod message {
    use context::Context;
    use state::State;
    use Message;

    #[derive(Debug)]
    pub struct Action;

    impl Action {
        pub fn execute(&self, state: &State, context: &Context) {
            println!("Message action executed");
        }
    }
}
