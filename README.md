# symfreq

[![Crates.io](https://img.shields.io/crates/v/symfreq.svg)](https://crates.io/crates/symfreq)

A CLI tool for analyzing symbol frequency in source code files.

## Motivation

Built to learn Rust while optimizing keyboard layouts for a custom split keyboard. This tool helps identify which
symbols appear most frequently in your codebase, making it easier to design efficient key placements.

## Installation

```bash
cargo install symfreq
```

## Usage

Analyze a directory with default extensions (rs, js, jsx, ts, tsx):

```bash
symfreq ./src
```

Specify custom file extensions:

```bash
symfreq ./src --exts rs,toml,md
```

Or use the short flag:

```bash
symfreq ./src -e c,h,cpp
```

## Output

The tool displays a table showing each symbol and its percentage frequency:

```
┌────────┬─────────┐
│ Symbol │ Percent │
├────────┼─────────┤
│ (      │ 15.23%  │
│ )      │ 15.20%  │
│ ,      │ 12.45%  │
│ :      │ 8.34%   │
└────────┴─────────┘
```

## Options

- `-e, --exts <EXTENSIONS>` - Comma-separated list of file extensions to analyze ("rs,ts,tsx")
- `-h, --help` - Print help
- `-V, --version` - Print version

## License

MIT