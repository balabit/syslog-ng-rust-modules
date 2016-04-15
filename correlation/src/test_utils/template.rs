use Template;
use Event;
use TemplateFactory;
use Message;
use CompileError;

use std::fmt::Write;
use std::sync::Arc;

pub struct MockTemplate {
    pub with_context: Box<Mock>,
}

pub trait Mock: Send {
    fn call(&self, messages: &[Arc<Message>], context_id: &str, &mut String);
}

/// implement Mock for bare fns
impl<F: Send + for<'a, 'b, 'c> Fn(&'a [Arc<Message>], &'b str, &'c mut String)> Mock for F {
    fn call(&self, messages: &[Arc<Message>], context_id: &str, buffer: &mut String) {
        (*self)(messages, context_id, buffer)
    }
}

struct LiteralMockTemplate(String);

impl Mock for LiteralMockTemplate {
    fn call(&self, _: &[Arc<Message>], _: &str, buffer: &mut String) {
        let _ = buffer.write_str(&self.0);
    }
}

impl MockTemplate {
    // return a literal from format()
    pub fn literal(value: &str) -> MockTemplate {
        MockTemplate {
            with_context: Box::new(LiteralMockTemplate(value.to_owned())),
        }
    }
}

impl Template for MockTemplate {
    type Event = Message;
    fn format_with_context(&self, messages: &[Arc<Self::Event>], context_id: &str, buffer: &mut String) {
        self.with_context.call(messages, context_id, buffer)
    }
}

pub struct MockTemplateFactory (Box<Fn(&str) -> Result<MockTemplate, CompileError>>);

impl MockTemplateFactory {
    // returns the value which is compiled as an error
    pub fn compile_error() -> MockTemplateFactory {
        MockTemplateFactory(Box::new(move |value| { Err(CompileError(value.to_owned())) }))
    }
    // returns the value used for compilation
    pub fn compile_value() -> MockTemplateFactory {
        MockTemplateFactory(Box::new(move |value| { Ok(MockTemplate::literal(value)) }))
    }
}

impl TemplateFactory<Message> for MockTemplateFactory {
    type Template = MockTemplate;
    fn compile(&self, value: &str) -> Result<MockTemplate, CompileError> {
        self.0(value)
    }
}

#[test]
fn test_mock_template_factory_can_generate_errors() {
    let factory = MockTemplateFactory::compile_error();
    let expected = CompileError("ERROR".to_owned());
    let actual = factory.compile("ERROR").err().unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_mock_template_factory_can_generate_template_which_returns_the_compiled_value() {
    let factory = MockTemplateFactory::compile_value();
    let expected = "VALUE";
    let template = factory.compile(expected).ok().unwrap();
    let dummy_context_id = "doesn't matter";
    let mut actual = String::new();
    template.format_with_context(&[], dummy_context_id, &mut actual);
    assert_eq!(expected, actual);
}

#[test]
fn test_mock_template_returns_the_expected_literal() {
    let expected = "literal";
    let template = MockTemplate::literal(expected);
    let dummy_context_id = "doesn't matter";
    let mut actual = String::new();
    template.format_with_context(&[], dummy_context_id, &mut actual);
    assert_eq!(expected, actual);
}
