# correlation

[![Coverage Status](https://coveralls.io/repos/github/ihrwein/correlation/badge.svg?branch=master)](https://coveralls.io/github/ihrwein/correlation?branch=master)

A library for grouping events based on predefined rules.

## Details

This library can group events based on their unique identifier or other
properties. Every event has a unique identifier and several key-value pairs.
You can turn your raw logs into events by parsing them. This library has a
"brother" library (https://github.com/ihrwein/actiondb-parser) which does exactly that: it parses raw messages based on
patterns, and converts them into key-value pairs.

You can use these libraries in syslog-ng (https://syslog-ng/org) to parse and correlate your log messages.

Note that both libraries are work in progress.

## Compilation

Just use this lib in your `Cargo.toml`.

## Use cases

Before diving into the configuration, let's see some use cases about what you can
accomplish with this library:

1. Message deduplication/suppression (any type, not just consecutive ones)
1. Generating alerts when some events occurred
1. Event transformation: receiving one type of event, generating a new one
1. [YOUR use case here: if you do something interesting with this library, let me know about it!]

## Contribution

If you find this library interesting and also want to try out the Rust systems
programming language, you are in a good place! Your contribution is greatly
appreciated and welcome! Every issue is mentored and has a difficulty level
assigned. If you don't find anything worth of your attention, please hit me up
(Cargo.toml has my mail address) and we will find an interesting task for you
(for example, threading).

### Configuration file format

The configuration file is a JSON file, basically it is an array of "context"
definitions. The following (overly complex, contains all possible
configuration options) JSON document is an example of this configuration
format:

```json
[
    {
        "name": "MAIL_READ",
        "uuid": "f7ee6a32-03a6-40d9-bd87-f48d1b4cd563",
        "patterns": [
          "LOGIN",
          "MAIL_READ",
          "LOGOUT"
        ],
        "conditions": {
            "first_opens": true,
            "last_closes": true,
            "timeout": 3600000,
            "renew_timeout": 100,
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
A "context" is a group of messages that belong together based on some property (for example, they are sent by the same application on the same host). You can
execute actions on the grouped events when some conditions are met. Currently the only
supported action type is generating an artificial log message.

The following fields can be used in a context definition:

* `name`: string, optional. The human readable name of the context (e.g. `SSH_LOGIN`).
* `uuid`: UUID, required. The unique identifier of the context definition.
* `patterns`: The context is interested in this list of event identifiers/names. If it's empty or not present, the
context is subscribed to all events.
* `conditions`: required. Defines how this context can be opened or closed:
 * `first_opens`: If `true`, the context is opened only when the first element of `patterns` list is received (that is, a message defines the beginning of the context, for example, a login message)
 * `last_closes`: If `true`, the last element of `patterns` closes the context (for example, if a logout message is received that matches the context)
 * `timeout`: After opening the context, it is automatically closed after `timeout` milliseconds.
 * `renew_timeout`: The context is closed if `renew_timeout` milliseconds elapses without receiving a new event to the context.
 * `max_size`: The maximal number of events this context can store.
* `context_id`: A Handlebars template which can be used to group events based on their key-value pairs. Every key-value
pair of an event can be used.
If two rendered template is the same for two events, they are grouped into the same context (of course,
  an event can belong to several contexts at the same time).
* `actions`: An array of several actions which are executed when the context is opened or closed.

| Name                     | Optional | Value type                   | Default value |
|--------------------------|----------|------------------------------|---------------|
| name                     | yes      | string                       |               |
| uuid                     | no       | UUID                         |               |
| patterns                 | yes      | array                        |               |
| conditions.timeout       | no       | int [ms]                     |               |
| conditions.renew_timeout | yes      | int [ms]                     |               |
| conditions.first_opens   | yes      | bool                         | false         |
| conditions.last_closes   | yes      | bool                         | true          |
| conditions.max_size      | yes      | int                          |               |
| context_id               | yes      | string [Handlebars template] |               |
| actions                  | yes      | array                        |               | |

#### Actions
There is one action type defined currently: `message`.
##### Message

The `message` action's definition is as follows:

* `uuid`: The unique identifier of the message which is generated,
* `name`: The human readable name of the message which is generated,
* `message`: A Handlebars template that represents the contents of the message (like the message portion of a raw syslog)
* `values`: Key-value pairs. The values are Handlebars templates.
* `inject_mode`: Represents how the generated message should be injected into the application. It has three distinct values:
 * `log`: Log the message (via standard `syslog()` call, through log4j, etc.)
 * `forward`: Forward the message to the next processing pipeline element.
 * `loopback`: Send the message back to the correlator engine for multi-layer correlation.
* `when`: Defines when the action should be executed
 * `on_opened`: When the context is opened
 * `on_closed`: When the context is closed.


 | Name           | Optional | Value type                               | Default value |
 |----------------|----------|------------------------------------------|---------------|
 | uuid           | no       | UUID                                     |               |
 | name           | yes      | string                                   |               |
 | message        | no       | string [Handlebars template]             |               |
 | values         | yes      | object (values are Handlebars templates) |               |
 | inject_mode    | yes      | enum (log,forward,loopback)              | log           |
 | when.on_opened | yes      | bool                                     | false         |
 | when.on_closed | yes      | bool                                     | true          |

The templatable values has read-only access to the state of the context when the message is rendered/
The following variables can be used in the message:

* `messages`: The array of events stored in the context
* `context_name`: The name of the generating context (if any)
* `context_uuid`: The uuid of the generating context
* `context_len`: The number of captured events.

A message is represented by a JSON object with the following keys:

* `uuid`: The uuid of the message
* `name`: The optional name of the message
* `message`: The message portion
* `values`: The key-value pairs stored in the message

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
for inclusion in the Work by You, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
