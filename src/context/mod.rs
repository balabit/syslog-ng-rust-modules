use dispatcher::request::Request;
use dispatcher::response::ResponseSender;

pub use self::linear::LinearContext;
pub use self::map::MapContext;
pub use self::base::BaseContext;
pub use self::base::BaseContextBuilder;
pub use self::context_map::ContextMap;

pub mod base;
pub mod context_map;
pub mod linear;
pub mod map;
#[cfg(test)]
mod test;

pub enum Context {
    Linear(LinearContext),
    Map(MapContext),
}
impl Context {
    pub fn on_event(&mut self, event: Request, responder: &mut ResponseSender) {
        match *self {
            Context::Linear(ref mut context) => context.on_event(event, responder),
            Context::Map(ref mut context) => context.on_event(event, responder),
        }
    }

    pub fn patterns(&self) -> &[String] {
        match *self {
            Context::Linear(ref context) => context.patterns(),
            Context::Map(ref context) => context.patterns(),
        }
    }
}
