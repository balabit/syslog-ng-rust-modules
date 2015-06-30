use logmsg::*;
use cfg::*;

#[repr(C)]
pub struct RustFilterProxy {
    pub filter: Box<RustFilter>
}

pub trait RustFilter {
    fn init(&mut self, _: &GlobalConfig) {}
    fn eval(&self, msg: &mut LogMessage) -> bool;
    fn set_option(&mut self, key: String, value: String);
}
