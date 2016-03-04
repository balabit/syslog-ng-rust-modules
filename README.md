# Filling the parse() method

My previous post described how to build a very simple parser plugin for
syslog-ng in Rust.  I brought a more realistic example this time: a regular
expression based [parser plugin](https://github.com/ihrwein/regex-parser).
It's so real that is is decently covered with unit tests and it has even a
benchmark.

For those who didn't read my last post, here are the most important steps to
get a working parser plugin in Rust:
1. create a lib crate
1. add dependencies
1. set the crate type
1. implement `Parser` and `ParserBuilder`
1. use the `parser_plugin!` macro
1. write the build script

If you are content with your work, you have to copy the library where syslog-ng
can find it.

That's enough for introduction, this time I'd like to concentrate on `Paser's`
`parse()` method.

It makes no sense to parse a log message but not to store the parsed
information.

syslog-ng represents log messages in a map like data structure. For example,
the `HOST` key represents the origin of the message, the `PROGRAM` key the name
of the program.  The `parse()` method takes a mutable reference to this map so
it can add new values to it.

It's interesting to know that the values can be accessed via two method:
* using string references as keys (like `HOST`)
* obtaining a persistent handle (during reloads) to a given entry
  (`LogMsg::get_value_handle()`).

Either you have a string or a handle, you can use the `get()` method to get a
value, it's generic over the concrete implementation.

The `insert()` method can be used to add values to the map. If you use handles
for the insertion, actually you can put data into the map without heap allocation.
When I made this optimization available for Rust, I was able to reduce the benchmark
result from `5,583 ns` to `4,717 ns` (parsing of `loggen's` output message).

You can find the code of the regex parser here:
https://github.com/ihrwein/regex-parser

Here is a simple `syslog-ng.conf` which can utilize this parser:

```
@version: 3.8

source s_localhost {
    network(
        ip(127.0.0.1),
        port(1514),
        transport("tcp")
    );
};

log {
    source(s_localhost);
    parser {
        regex-rs(
            # Note the additional escaping before the backslashes!!!
            option("regex", "seq: (?P<seq>\\d+), thread: (?P<thread>\\d+), runid: (?P<runid>\\d+), stamp: (?P<stamp>[^ ]+) (?P<padding>.*$)")
        );
    };
    destination {
        file("/dev/stdout" template("runid=$runid\n"));
    };
};
```

You can use syslog-ng's bundled `loggen` command to generate a sample log

```
$ loggen -S -n 10 127.0.0.1 1514
```

GSoC advertisement: if you are a student and want to contribute to syslog-ng in
Google Summer of Code, you can find our project page
[here](https://github.com/balabit/syslog-ng/wiki/GSoC2016).
