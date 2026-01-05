# symfreq

[![Crates.io](https://img.shields.io/crates/v/symfreq.svg)](https://crates.io/crates/symfreq)

A CLI tool for analyzing symbol frequency in source code files.

## Motivation

Built to learn Rust while optimizing keyboard layouts for a custom split keyboard. This tool helps identify which
symbols appear most frequently in your codebase, making it easier to design efficient key placements.

## Installation

### With Homebrew (macOS/Linux)

```bash
brew tap vaskort/symfreq
brew install symfreq
```

### With Cargo

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

The tool displays a summary with statistics and a color-coded table showing each symbol's frequency:

```
Summary:
  Total tracked symbols: 1159813
  Unique symbols: 32
  Files processed: 2042 read (51.3%), 1940 skipped, 0 failed

┌────────┬─────────┬───────────────────────────┐
│ Symbol │ Percent │ Distribution              │
├────────┼─────────┼───────────────────────────┤
│ "      │ 12.26%  │ ███░░░░░░░░░░░░░░░░░░░░░░ │
│ (      │ 11.85%  │ ██░░░░░░░░░░░░░░░░░░░░░░░ │
│ )      │ 11.85%  │ ██░░░░░░░░░░░░░░░░░░░░░░░ │
│ .      │ 9.64%   │ ██░░░░░░░░░░░░░░░░░░░░░░░ │
│ _      │ 9.19%   │ ██░░░░░░░░░░░░░░░░░░░░░░░ │
└────────┴─────────┴───────────────────────────┘
```

### Color Coding

- **Green** - High frequency symbols (≥10%)
- **Yellow** - Medium frequency symbols (5-10%)
- **White** - Low frequency symbols (<5%)

## Options

- `-e, --exts <EXTENSIONS>` - Comma-separated list of file extensions to analyze ("rs,ts,tsx")
- `-h, --help` - Print help
- `-V, --version` - Print version

## License

MIT
