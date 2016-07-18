# Python parser for syslog-ng

For a real world tested example, check the `_test_module/regex.py` file.

A Python parser is a Python class which implements two methods:
* `init(self, options)`: (optional) After the parser instance was created, this method
  is called on it. The `options` variable is a dictionary with key-value pairs
* `parse(self, logmsg, message)`: (mandatory) This method is called upon receiving
 a new log mesage. The first, `logmsg` parameter is a dictionary-like data-structure
 which contains the already parsed key-value pairs. You can fetch the existing ones
 or insert new ones with the `__getitem__/__setitem__` methods.

Example:
```python
import re

class RegexParser:
    def init(self, options):
        pattern = options["regex"]
        self.regex = re.compile(pattern)

    def parse(self, logmsg, message):
        match = self.regex.match(message)
        if match is not None:
            for key, value in match.groupdict().items():
                logmsg[key] = value
            return True
        else:
            return False
```

If an exception is thrown during `init()` is is considered an initialization error and syslog-ng won't be started.

## Configuration


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
    source(
        s_localhost
    );
    parser {
        python(
            module("_test_module.regex")
            class("RegexParser")
            option("regex", "seq: (?P<seq>\\d+), thread: (?P<thread>\\d+), runid: (?P<runid>\\d+), stamp: (?P<stamp>[^ ]+) (?P<padding>.*$)")
        );
    };
    destination {
        file("/dev/stdout" template("runid=$runid\n"));
    };
};
```

Make sure, that you can import the `_test_module.regex` module
from a Python shell. If not, you can add its directory to
the `PYTHONPATH` environment variable:

```
PYTHONPATH=/home/tibi/workspace/python-parser sbin/syslog-ng -Fevd
```

## Compilation

You need a nightly Rust compiler.
Make sure, pkg-config is able to find syslog-ng and `libsyslog-ng.so` is in your
library path.

```
cargo build --release
cp target/release/libpython_parser.so <syslog-ng install prefix>/lib/syslog-ng/
```
