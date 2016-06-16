use Template;
use TemplateFactory;
use Message;
use CompileError;

use std::io::Write;

pub struct MockTemplate {
    pub with_context: Box<Mock>,
}

pub trait Mock: Send {
    fn call(&self, messages: &[Message], context_id: &str, &mut Write);
}

/// implement Mock for bare fns
impl<F: Send + for<'a, 'b, 'c> Fn(&'a [Message], &'b str, &'c mut Write)> Mock for F {
    fn call(&self, messages: &[Message], context_id: &str, buffer: &mut Write) {
        (*self)(messages, context_id, buffer)
    }
}

struct LiteralMockTemplate(Vec<u8>);

impl Mock for LiteralMockTemplate {
    fn call(&self, _: &[Message], _: &str, buffer: &mut Write) {
        let _ = buffer.write(&self.0);
    }
}

fn context_id(_: &[Message], context_id: &str, buffer: &mut Write) {
    let _ = buffer.write(context_id.as_bytes());
}

fn context_len(messages: &[Message], _: &str, buffer: &mut Write) {
    let _ = buffer.write_fmt(format_args!("{}", messages.len()));
}

impl MockTemplate {
    // return a literal from format()
    pub fn literal(value: &[u8]) -> MockTemplate {
        MockTemplate {
            with_context: Box::new(LiteralMockTemplate(value.to_vec())),
        }
    }
    pub fn context_id() -> MockTemplate {
        MockTemplate {
            with_context: Box::new(context_id),
        }
    }
    pub fn context_len() -> MockTemplate {
        MockTemplate {
            with_context: Box::new(context_len),
        }
    }
}

impl Template for MockTemplate {
    type Event = Message;
    fn format_with_context(&self, messages: &[Self::Event], context_id: &str, buffer: &mut Write) {
        self.with_context.call(messages, context_id, buffer)
    }
}

pub struct MockTemplateFactory (Box<Fn(&[u8]) -> Result<MockTemplate, CompileError>>);

impl MockTemplateFactory {
    // returns the value which is compiled as an error
    pub fn compile_error() -> MockTemplateFactory {
        MockTemplateFactory(Box::new(move |value| { Err(CompileError(value.to_vec())) }))
    }
    // returns the value used for compilation
    pub fn compile_value() -> MockTemplateFactory {
        MockTemplateFactory(Box::new(move |value| { Ok(MockTemplate::literal(value)) }))
    }
}

impl TemplateFactory<Message> for MockTemplateFactory {
    type Template = MockTemplate;
    fn compile(&self, value: &[u8]) -> Result<MockTemplate, CompileError> {
        self.0(value)
    }
}

#[test]
fn test_mock_template_factory_can_generate_errors() {
    let factory = MockTemplateFactory::compile_error();
    let expected = CompileError(b"ERROR".to_vec());
    let actual = factory.compile(b"ERROR").err().unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_mock_template_factory_can_generate_template_which_returns_the_compiled_value() {
    let factory = MockTemplateFactory::compile_value();
    let expected = b"VALUE";
    let template = factory.compile(expected).ok().unwrap();
    let dummy_context_id = "doesn't matter";
    let mut actual = Vec::new();
    template.format_with_context(&[], dummy_context_id, &mut actual);
    assert_eq!(expected, &actual[..]);
}

#[test]
fn test_mock_template_returns_the_expected_literal() {
    let expected = b"literal";
    let template = MockTemplate::literal(expected);
    let dummy_context_id = "doesn't matter";
    let mut actual = Vec::new();
    template.format_with_context(&[], dummy_context_id, &mut actual);
    assert_eq!(expected, &actual[..]);
}

#[test]
fn test_mock_template_can_return_context_id() {
    let context_id = "79ace9c4-0693-4d5b-97d8-de39322bc64d";
    let template = MockTemplate::context_id();
    let mut actual = Vec::new();
    template.format_with_context(&[], context_id, &mut actual);
    assert_eq!(context_id.as_bytes(), &actual[..]);
}

#[test]
fn test_mock_template_can_return_context_length() {
    let expected = b"0";
    let template = MockTemplate::context_len();
    let mut actual = Vec::new();
    template.format_with_context(&[], "doesn't matter", &mut actual);
    assert_eq!(expected, &actual[..]);
}
