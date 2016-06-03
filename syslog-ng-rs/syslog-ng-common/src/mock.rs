use Pipe;
use LogMessage;

pub struct MockPipe{
    pub forwarded_messages: Vec<LogMessage>
}

impl MockPipe {
    pub fn new() -> MockPipe {
        MockPipe{ forwarded_messages: Vec::new()}
    }
}

impl Pipe for MockPipe {
    fn forward(&mut self, msg: LogMessage) {
        self.forwarded_messages.push(msg);
    }
}
