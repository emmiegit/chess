## mallard-chess

<p align="center">
  <a href="https://github.com/ammongit/chess/actions?query=workflow%3A%22Build%22">
    <img src="https://github.com/ammongit/chess/workflows/Build/badge.svg"
         alt="Build status">
  </a>
</p>

Chess engine wrapper utility created for fun. Intended for use with xboard; utilizes the [Universal Chess Interface](https://en.wikipedia.org/wiki/Universal_Chess_Interface).

## Requirements

Because the engine uses [stockfish](https://stockfishchess.org/) for actual game engine decisioning, it must be installed and available in your `$PATH`.

This code should be portable, but I do not use Windows and am not confident it will work flawlessly.

## Execution

```
cargo run --release -- [options]
```

Run with `--help` for command-line usage.
