# locc

[![crates.io](https://img.shields.io/crates/v/locc.svg)](https://crates.io/crates/locc)

Count lines of code per language.
Respects `.gitignore` and skips hidden files.

```
cargo install locc
```

```
$ locc --detailed
████████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   Language           Lines  Percentage
█  Rust              12345    85.3%
█  Python             2345    10.2%
█  TypeScript          987     4.2%
```

## Usage

```
locc [OPTIONS] [PATH]
```

| Flag | Description |
|------|-------------|
| (none) | Language, lines, percentage |
| `--short` | Language and percentage only |
| `--lines` | Language and lines only |
| `--graph` | Colored breakdown bar |
| `--detailed` | Bar, header, language, lines, percentage |

Flags can be combined (e.g. `--graph --short`).
