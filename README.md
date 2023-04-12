Tic Tac Toe aka knots and crosses in Rust
=========================================

This is an educational project for me to learn Rust with.

Setup
-----

```bash
git clone https://github.com/InDieTasten/tictactoe-rs
cd tictactoe-rs
cargo build
```

Playing
-------

## Hosting a match using `play`

### Usage from `cargo run -- play --help`
```
Usage: tictactoe-rs play <X> <O>

Arguments:
  <X>  [possible values: local, ai]
  <O>  [possible values: local, ai]

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Examples

Local player vs local player
```bash
cargo run -- play local local
```

Local player vs AI
```bash
cargo run -- play local ai
```

AI vs AI
```bash
cargo run -- play ai ai
```
