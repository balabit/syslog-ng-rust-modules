# correlation

A library for grouping events based on predefined rules.

## Details

This library is able to group events based on their unique identifier or other
properties. Every event has a unique identifier and several key-value pairs.
You can turn your raw logs into events by parsing them. This library has a
"brother" library which does exactly this: parses raw messages based on
patterns. syslog-ng can use this parsing library, you can find more information
here: https://github.com/ihrwein/actiondb-parser

The correlation-parser module for syslog-ng is work in progress.

## Use cases

Before diving into the configuration, let's see some use cases which you can
accomplish by using this library:

1. Message deduplication/suppression (any type, not just consecutive ones)
1. Generating alerts when some events occurred
1. Event transformation: receiving one type of event, generating a new one
1. [YOUR use case here, I'd like to hear it!]

## Contribution

If you find this library interesting and also want to try out the Rust systems
programming language, you are in a good place! Your contribution is very
appreciated and welcomed! Every issue is mentored and has a difficulty level
assigned. If you don't find anything worth of your attention, please hit me up
(Cargo.toml has my mail address) and we will find something interesting
(threading, etc.).

### Configuration file format

The configuration file is a JSON file and basically an array of "context"
definitions.  The following (overly complex, contains all possible
configuration options) JSON document is an example of this configuration
format:

```json
[
    {
        "name": "MAIL_READ",
        "uuid": "f7ee6a32-03a6-40d9-bd87-f48d1b4cd563",
        "conditions": {
            "timeout": 3600000,
            "renew_timeout": 100,
            "patterns": [
                "LOGIN",
                "MAIL_READ",
                "LOGOUT"
            ],
            "first_opens": true,
            "last_closes": true,
            "max_size": 5
        },
        "context_id": "{{user_name}}",
        "actions": [
            {
                "message": {
                    "uuid": "4bbd15c4-ec44-47a2-ada3-f7fe3ff81222",
                    "name": "MAIL_READ",
                    "message": "message",
                    "values": {
                        "MESSAGE": "user logged in, read mails then logged out"
                    },
                    "when": {
                      "on_opened": false,
                      "on_closed": true
                    }
                }
            }
        ]
    }
]
```
A "context" defines what kind of events and how should be grouped. You can
execute actions on the grouped events when some conditions are met. The only
supported action type is generating an artificial message.

The following fields can be used in a context definition:
* `name`: string, optional. The human readable name of the context (e.g. `SSH_LOGIN`).
* `uuid`: UUID, required. The unique identifier of the context definition.
* `conditions`: required. Defines how this context can be opened or closed:
 * `timeout`: after `timeout` milisecs of opening the context it has to be closed.
 * `renew_timeout`: if `renew_timeout` milisecs elapses without receiving a new event the context has to be closed.
 * `patterns`: the context is interested in this list of event identifiers/names. If it's empty or not present, the
 context is subscribed to all events.
 * `first_opens`: only the first element of `patterns` can open the context_id
 * `last_closes`: the last element of `patterns` can also close the context
 * `max_size`: the maximal number of events this context can store.
* `context_id`: a Handlebars template which can be used to group events based on their key-value pairs. Every key-value
pair of an event can be used.
If two rendered template is the same for two events, they are grouped into the same context (of couse,
  an event can belong to several contexts in the same time).
* `actions`: an array of several actions which can be run on the opening or the closing
of the context.

| Name                     | Optional | Value type                   | Default value |
|--------------------------|----------|------------------------------|---------------|
| name                     | yes      | string                       |               |
| uuid                     | no       | UUID                         |               |
| conditions.timeout       | no       | int [ms]                     |               |
| conditions.renew_timeout | yes      | int [ms]                     |               |
| conditions.patterns      | yes      | array                        |               |
| conditions.first_opens   | yes      | bool                         | false         |
| conditions.last_closes   | yes      | bool                         | true          |
| conditions.max_size      | yes      | int                          |               |
| context_id               | yes      | string [Handlebars template] |               |
| actions                  | yes      | array                        |               | |

#### Actions
There is one action type defined currently: `message`.
##### Message

The `message` action's definition is as follows:
* `uuid`: the unique identifier of the event which is generated,
* `name`: the human readable name of the event which is generated,
* `message`: a Handlebars template, represents the contents of the message (like the message portion of a raw syslog)
* `values`: key-value pairs. The values are Handlebars templates.
* `inject_mode`: represents how the generated message should be injected into the application. It has three distinct values:
 * `log`: the message should be logged (via standard `syslog()` call, through log4j, etc.)
 * `forward`: the message should be forwarded to the next processing pipeline element,
 * `loopback`: the message should be looped back to the correlator engine for multi-layer correlation.
* `when`: defines when the action should be executed
 * `on_opened`: when the context is being closed,
 * `on_closed`: when the context is being opened.


 | Name           | Optional | Value type                               | Default value |
 |----------------|----------|------------------------------------------|---------------|
 | uuid           | no       | UUID                                     |               |
 | name           | yes      | string                                   |               |
 | message        | no       | string [Handlebars template]             |               |
 | values         | yes      | object (values are Handlebars templates) |               |
 | inject_mode    | yes      | enum (log,forward,loopback)              | log           |
 | when.on_opened | yes      | bool                                     | false         |
 | when.on_closed | yes      | bool                                     | true          |

The templatable values has read only access to the state of the context during
rendering. The following variables can be used during rendering:

* `messages`: the array of events captured by this context,
* `context_name`: the name of the generating context (if any)
* `context_uuid`: the uuid of the generating context
* `context_len`: the number of captured events.

A message is represented by a JSON object with the following keys:
* `uuid`: the uuid of the message,
* `name`: the optional name of the message,
* `message`: the message portion,
* `values`: the key-value pairs stored in the message.

The following Handlebars library is used:
https://github.com/sunng87/handlebars-rust. You can use all the syntax it
supports.

Examples:
```
key1={{{messages.[0].values.key1}}}
we have {{context_len}} messages
```

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
