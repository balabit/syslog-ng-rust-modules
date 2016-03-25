# actiondb-parser

A fast and easy way to parse your logs into events.

## Requirements

* a C compiler installed
* rustc and cargo installed (tested from 1.5, but it can work with older versions)
* pkg-config installed
* syslog-ng 3.8 installed and can be found by pkg-config

## Usage

You have to compile the source code and copy the resulted shared library into a location
which is looked up by syslog-ng.

1. Compilation:

 ```
git clone https://github.com/ihrwein/actiondb-parser.git
cd actiondb-parser
cargo build --release
```

2. Copy the `libactiondb_parser.so` file next to `libcsvparser.so` (that's the easiest way to find
the proper directory)

 ```
cp target/release/libactiondb_parser.so <target directory>
```

3. You can use it immediately:

 ```
    parser{
        actiondb-rs(
            # the patterns will be loaded from this file
            option("pattern_file", "/home/tibi/install/syslog-ng/etc/loggen.json")
            # all the parsed keys should be prefixed with `.adb`
            option("prefix", ".adb")
        );
    };
```

Note, that in order to use the parser, you don't need the Rust runtime, it's already compiled
into the shared library. Check the required libraries with `ldd`. I suppose still you have to install
`libgcc1`.

For the exact configuration file format, check ActionDB's readme file: https://github.com/ihrwein/actiondb/blob/master/README.md

## adbtool

`adbtool` is a tool which can be used for the following purposes:
* validate patterns,
* parse text files.

It support the `validate` and `parse` subcommands. For more information check
it's `--help` option.

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
