use syslog_ng_common::formatter::MessageFormatter;
use syslog_ng_common::LogMessage;
use actiondb::matcher::result::MatchResult;

use keys;

pub struct MessageFiller;

impl MessageFiller {
    pub fn fill_logmsg(formatter: &mut MessageFormatter, msg: &mut LogMessage, result: &MatchResult) {
        MessageFiller::fill_values(formatter, msg, result);
        MessageFiller::fill_name(formatter, msg, result);
        MessageFiller::fill_uuid(formatter, msg, result);
        MessageFiller::fill_tags(msg, result);
    }

    fn fill_values(formatter: &mut MessageFormatter, msg: &mut LogMessage, result: &MatchResult) {
        MessageFiller::fill_parsed_values(formatter, msg, result);
        MessageFiller::fill_additional_values(formatter, msg, result);
    }

    fn fill_parsed_values(formatter: &mut MessageFormatter, msg: &mut LogMessage, result: &MatchResult) {
        for (key, value) in result.values() {
            let (key, value) = formatter.format(key, value);
            msg.set_value(key, value);
        }
    }

    fn fill_additional_values(formatter: &mut MessageFormatter, msg: &mut LogMessage, result: &MatchResult) {
        if let Some(values) = result.pattern().values() {
            for (key, value) in values {
                let (key, value) = formatter.format(key, value);
                msg.set_value(key, value);
            }
        }
    }

    fn fill_name(formatter: &mut MessageFormatter, msg: &mut LogMessage, result: &MatchResult) {
        if let Some(name) = result.pattern().name() {
            let (key, value) = formatter.format(keys::PATTERN_NAME, name);
            msg.set_value(key, value);
        }
    }

    fn fill_uuid(formatter: &mut MessageFormatter, msg: &mut LogMessage, result: &MatchResult) {
        let uuid = result.pattern().uuid().to_hyphenated_string();
        let (key, value) = formatter.format(keys::PATTERN_UUID, &uuid);
        msg.set_value(key, value);
    }

    fn fill_tags(msg: &mut LogMessage, result: &MatchResult) {
        if let Some(tags) = result.pattern().tags() {
            for i in tags {
                msg.set_tag(i);
            }
        }
    }
}
