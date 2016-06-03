// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use syslog_ng_common::MessageFormatter;
use syslog_ng_common::LogMessage;
use actiondb::matcher::result::MatchResult;

use keys;

pub struct MessageFiller;

impl MessageFiller {
    pub fn fill_logmsg(formatter: &mut MessageFormatter,
                       msg: &mut LogMessage,
                       result: &MatchResult) {
        MessageFiller::fill_values(formatter, msg, result);
        MessageFiller::fill_name(formatter, msg, result);
        MessageFiller::fill_uuid(formatter, msg, result);
        MessageFiller::fill_tags(msg, result);
    }

    fn fill_values(formatter: &mut MessageFormatter, msg: &mut LogMessage, result: &MatchResult) {
        MessageFiller::fill_parsed_values(formatter, msg, result);
        MessageFiller::fill_additional_values(formatter, msg, result);
    }

    fn fill_parsed_values(formatter: &mut MessageFormatter,
                          msg: &mut LogMessage,
                          result: &MatchResult) {
        for (key, value) in result.values() {
            let (key, value) = formatter.format(key, value);
            msg.insert(key, value.as_bytes());
        }
    }

    fn fill_additional_values(formatter: &mut MessageFormatter,
                              msg: &mut LogMessage,
                              result: &MatchResult) {
        if let Some(values) = result.pattern().values() {
            for (key, value) in values {
                let (key, value) = formatter.format(key, value);
                msg.insert(key, value.as_bytes());
            }
        }
    }

    fn fill_name(formatter: &mut MessageFormatter, msg: &mut LogMessage, result: &MatchResult) {
        if let Some(name) = result.pattern().name() {
            let (key, value) = formatter.format(keys::PATTERN_NAME, name);
            msg.insert(key, value.as_bytes());
        }
    }

    fn fill_uuid(formatter: &mut MessageFormatter, msg: &mut LogMessage, result: &MatchResult) {
        let uuid = result.pattern().uuid().hyphenated().to_string();
        let (key, value) = formatter.format(keys::PATTERN_UUID, &uuid);
        msg.insert(key, value.as_bytes());
    }

    fn fill_tags(msg: &mut LogMessage, result: &MatchResult) {
        if let Some(tags) = result.pattern().tags() {
            for i in tags {
                msg.set_tag(i.as_bytes());
            }
        }
    }
}
