//! Contains mock types to lessen the coupling with syslog-ng.

use Pipe;
use LogMessage;

/// Type useful for mocking any `Pipe`.
///
/// It will store the messages in an internal vector.
pub struct MockPipe{
    pub forwarded_messages: Vec<LogMessage>
}

impl MockPipe {
    /// Creates a new `MockPipe`.
    pub fn new() -> MockPipe {
        MockPipe{ forwarded_messages: Vec::new()}
    }
}

impl Pipe for MockPipe {
    /// Stores the messages in the internal vector.
    fn forward(&mut self, msg: LogMessage) {
        self.forwarded_messages.push(msg);
    }
}
