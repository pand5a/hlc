## Background

Reading the Cockroachdb code and during the debugging process, you will see many printed HLC timestamps, which sometimes need to be converted to local time, hence this mini program.

Support HLC timestamps in the following formats

```text
1657662798.107432301,2147483647
1657662798.107432301.2147483647
1657662798107432301,2147483647
1657662798107432301.2147483647
1657662798.107432301
1657662798107432301
1708329839396 // unix timestamp
```

## Build

```shell
cargo build --release
```

## Usage

```shell
USAGE:
    hlc [FLAGS] <input>

FLAGS:
    -h, --help       Prints help information
    -u               show utc date
    -V, --version    Prints version information

ARGS:
    <input>    hlc timestamp
```

## Example

```shell
// Local datetime
hlc 1657662798.107432301,2147483647

// UTC datetime
hlc -u 1657662798.107432301,2147483647

// output
2022-07-12 21:53:18.107432301, 1657662798107432301
```

