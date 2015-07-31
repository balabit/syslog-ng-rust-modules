# Proposal about the new correlation parser
I would like to propose a new correlation parser for syslog-ng which can replace
the correlation feature in PatternDB.

## Causes
* correlation and parsing are not separated parsers
* correlation cannot be configured without parser

## Proposal
I propose a Rust based correlation parser. Its advantages are:
* it's designed from scratch with simplicity in mind
* it tries to avoid the mistakes we learned from PatternDB
* it's independent from the other parsers like PatternDB or ActionDB

It is implemented as a Rust library which is used by a syslog-ng parser. We already
have one parser implemented in Rust so the bindings are ready.

## Architecture

### Components
The library has the following components:
* `Correlator` module which initializes the other components and starts the threads. It's connected to the `Dispatcher` and sends `Message` events to it through a synchronous channel.
* `Timer`: periodically emits `TimerEvent`s which stores the elapsed time. The events are sent to the `Dispatcher` through a synchronous channel.
* `Dispatcher`: dispatches the events (`Message` or `Timer`) to all `Context` instances
* `Context`: provides an interface to handle events (`Message` and `Timer`)

### Dynamic behavior
The library spawns two threads (at least):
* `Timer` has its own thread
* `Dispatcher` runs in its own thread
