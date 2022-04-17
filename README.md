## mallard-chess

<a href="https://github.com/ammongit/chess/actions?query=workflow%3A%22Build%22">
  <img src="https://github.com/ammongit/chess/workflows/Build/badge.svg"
       alt="Build status">
</a>

Chess engine wrapper created for fun. Intended for use with xboard; utilizes the [Universal Chess Interface](https://en.wikipedia.org/wiki/Universal_Chess_Interface).

## Requirements

Because the engine uses [stockfish](https://stockfishchess.org/) for actual game engine decisioning, it must be installed and available in your `$PATH`.

This code should be portable, but I do not use Windows and am not confident it will work flawlessly.

## Execution

```
cargo run --release -- [options] <engine>
```

Run with `--help` for command-line usage. Current engines are:
* `stockfish` (Run Stockfish normally)
* `worstfish` (Use Stockfish to choose the worst-scoring moves)
* `random` (Choose moves at random)
