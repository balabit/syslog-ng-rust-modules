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

impl Clone for RustParserProxy {
    fn clone(&self) -> RustParserProxy {
        trace!("Cloning RustParserProxy");
        RustParserProxy{
            parser: self.parser.boxed_clone()
        }
    }
}

pub trait RustParser {
    fn init(&mut self) -> bool { true }
    fn set_option(&mut self, _: String, _: String) {}
    fn process(&self, msg: &mut LogMessage, input: &str) -> bool;
    fn boxed_clone(&self) -> Box<RustParser>;
}
