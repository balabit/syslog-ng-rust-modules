# Filling the parse() method in syslog-ng’s Rust bindings

My [previous post](https://syslog-ng.org/syslog-ng-and-rust/) described how to build a very simple parser plugin for
syslog-ng in Rust. I brought a more realistic example this time: a regular
expression based [parser plugin](https://github.com/ihrwein/regex-parser).
It's so real that it is decently covered with unit tests and it has even a
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

That's enough for introduction, this time I'd like to concentrate on `Parser's`
`parse()` method.

It makes no sense to parse a log message without storing the parsed
information.

syslog-ng represents log messages in a map-like data structure. For example,
the `HOST` key represents the origin of the message, the `PROGRAM` key the name
of the program. The `parse()` method takes a mutable reference to this map so
it can add new values to it.

It's interesting to know that the values can be accessed via two method:
* using string references as keys (like `HOST`)
* obtaining a persistent handle (during reloads) to a given entry
  (`LogMsg::get_value_handle()`).

Either you have a string or a handle, you can use the `get()` method to get a
value, it's generic over the concrete implementation.

The `insert()` method can be used to add values to the map. If you use handles
for the insertion, you can put data into the map without heap allocation.
When I made this optimization available for Rust, I was able to reduce the benchmark
result from `5,583 ns` to `4,717 ns` (parsing `loggen's` output message).

You can find the code of the regex parser here:
https://github.com/ihrwein/regex-parser

## What does this parser do?

This parser takes a regular expression as a configuration parameter and matches
log messages against this expression. If a message successfully matches,
the parser extracts the named captures and inserts them into the data map of the log message.
You can use the names of the captures like any other name-value pair of syslog-ng, and use them
in filters, templates, and so on.

## If you want to try it out ...

Just follow these steps to get it working:

1. Compile and install the development version of syslog-ng. You can find
more instructions [here](https://github.com/balabit/syslog-ng#installation-from-source).
1. Append the directory of `libsyslog-ng.so` to your `LD_LIBRARY_PATH` environment variable
1. Clone the source code of [regex-parser](https://github.com/ihrwein/regex-parser)
1. (Optional) Build and run the unit tests with `cargo test`
1. (Optional, you will need a nightly compiler) Run the benchmark with `cargo bench`
1. Create a release build with `cargo build --release`
1. Copy the module to `$prefix/lib/syslog-ng` (where `$prefix` is the installation directory of syslog-ng)

```
cp target/release/libregex_parser.so <install prefix>lib/syslog-ng/
```

Here is a simple `syslog-ng.conf` that uses this parser.
The strings before the colons will be the names of the matches
extracted into name-value pairs (for example, `seq`, or `runid`).

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

Start syslog-ng in foreground mode (`syslog-ng -F`), then use the `loggen` command to generate a sample log message,
like `seq: 0000000000, thread: 0000, runid: 1456947132, stamp: 2016-03-02T20:32:12 PAD`:

```
$ loggen -S -n 10 127.0.0.1 1514
```

Syslog-ng's output should be something like this:

```
[2016-03-07T19:03:43.029114] Trying to compile regular expression: 'seq: (?P<seq>\d+), thread: (?P<thread>\d+), runid: (?P<runid>\d+), stamp: (?P<stamp>[^ ]+) (?P<padding>.*$)';
[2016-03-07T19:03:43.030100] Compiling #unnamed sequence [log] at [/home/tibi/install/syslog-ng/etc/syslog-ng.conf:17:5]
...
[2016-03-07T19:03:43.030180]       Compiling #unnamed single [log] at [/home/tibi/install/syslog-ng/etc/syslog-ng.conf:41:9]
[2016-03-07T19:03:43.030429] Building Regex parser;
[2016-03-07T19:03:43.030571] Running application hooks; hook='1'
[2016-03-07T19:03:43.030584] Running application hooks; hook='3'
[2016-03-07T19:03:43.030614] syslog-ng starting up; version='3.8.0alpha0'
```

GSoC advertisement: if you are a student and want to contribute to syslog-ng in
Google Summer of Code, you can find our project page
[here](https://github.com/balabit/syslog-ng/wiki/GSoC2016).
