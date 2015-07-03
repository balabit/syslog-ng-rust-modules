use logmsg::*;

#[repr(C)]
pub struct RustParserProxy {
    pub parser: Box<RustParser>
}

impl RustParserProxy {
    pub fn new(parser: Box<RustParser>) -> RustParserProxy {
        RustParserProxy{ parser: parser }
    }
}

pub trait RustParser {
    fn init(&mut self) -> bool { true }
    fn set_option(&mut self, _: String, _: String) {}
    fn process(&self, msg: &mut LogMessage, input: &str) -> bool;
}
