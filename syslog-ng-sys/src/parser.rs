use logmsg::*;

#[repr(C)]
pub struct RustParserProxy {
    pub filter: Box<RustFilter>
}

pub trait RustFilter {
    fn init(&mut self) {}
    fn process(&self, msg: &mut LogMessage, input: &str) -> bool;
    fn set_option(&mut self, key: String, value: String);
}
