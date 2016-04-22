# actiondb-parser

A fast and easy way to parse your logs into events.

## Usage

Sample syslog-ng configuration:

```
parser{
  actiondb(
    # the patterns will be loaded from this file
    pattern_file("/home/btibi/install/syslog-ng/etc/loggen.yaml")
    # all the parsed keys should be prefixed with `.adb`
    prefix(".adb")
  );
};
```

`loggen.yaml`:


```yaml
patterns:
  -
    uuid: "6d2cba0c-e241-464a-89c3-8035cac8f73e"
    name: "LOGGEN"
    pattern: "seq: %{INT:.loggen.seq}, thread: %{INT:.loggen.thread}, runid: %{INT:.loggen.runid}, stamp: %{GREEDY:.loggen.stamp} %{GREEDY:.loggen.padding}"
    tags:
      - "tag1"
      - "tag2"
    values:
      key1: "value1"
      key2: "value2"
    test_messages:
      -
        message: "seq: 0000000001, thread: 0000, runid: 1437655178, stamp: 2015-07-23T14:39:38 PADDPADDPADDPADD"
        values:
          .loggen.seq: "0000000001"
          .loggen.thread: "0000"
          .loggen.runid: "1437655178"
          .loggen.stamp: "2015-07-23T14:39:38"
          .loggen.padding: "PADDPADDPADDPADD"
        tags:
          - "tag1"
```



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
