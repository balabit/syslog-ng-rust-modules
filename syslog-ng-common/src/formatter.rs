use std::fmt::Write;

#[derive(Clone)]
pub struct MessageFormatter {
    buffer: String,
    prefix: Option<String>,
}

impl MessageFormatter {
    pub fn new() -> MessageFormatter {
        MessageFormatter {
            buffer: String::new(),
            prefix: None,
        }
    }

    pub fn set_prefix(&mut self, prefix: String) {
        self.prefix = Some(prefix)
    }

    pub fn format<'a, 'b, 'c>(&'a mut self, key: &'b str, value: &'c str) -> (&'a str, &'c str) {
        self.buffer.clear();
        self.apply_prefix(key);
        (&self.buffer, value)
    }

    fn apply_prefix(&mut self, key: &str) {
        match self.prefix.as_ref() {
            Some(prefix) => {
                let _ = self.buffer.write_str(prefix);
                let _ = self.buffer.write_str(key);
            }
            None => {
                let _ = self.buffer.write_str(key);
            }
        };
    }
}
