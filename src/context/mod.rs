use dispatcher::request::InternalRequest;

pub use self::linear::LinearContext;
pub use self::map::MapContext;
pub use self::base::BaseContext;
pub use self::base::BaseContextBuilder;
pub use self::context_map::ContextMap;

pub mod base;
pub mod context_map;
pub mod event;
pub mod linear;
pub mod map;
#[cfg(test)]
mod test;

pub enum Context {
    Linear(LinearContext),
    Map(MapContext)
}

impl From<Context> for Box<self::event::EventHandler<InternalRequest>> {
    fn from(context: Context) -> Box<self::event::EventHandler<InternalRequest>> {
        match context {
            Context::Linear(context) => Box::new(context),
            Context::Map(context) => Box::new(context),
        }
    }
}
