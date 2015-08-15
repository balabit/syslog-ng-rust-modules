use std::sync::mpsc::{Receiver, Sender};
use std::rc::Rc;

use action::ExecResult;
use super::{config, Condition, Context, Message, TimerEvent};
use reactor::{Event, EventDemultiplexer, EventHandler, Reactor};

use self::request::{Request, RequestHandler};

pub mod demux;
pub mod handlers;
pub mod request;
pub mod reactor;

#[derive(Debug)]
pub enum Response {
    Event(ExecResult),
    Exit
}
