## mallard-chess

<a href="https://github.com/emmiegit/chess/actions?query=workflow%3A%22Build%22">
  <img src="https://github.com/emmiegit/chess/workflows/Build/badge.svg"
       alt="Build status">
</a>

Chess engine wrapper created for fun. Intended for use with xboard; utilizes the [Universal Chess Interface](https://en.wikipedia.org/wiki/Universal_Chess_Interface).

To use it in xboard, add engine lines similar to the following:
```
"Pacifist" -fcp "mallard-chess pacifist" -fUCI
"Worstfish" -fcp "mallard-chess -N 10000 worstfish" -fUCI
```

These chess engines are my implementation of some of the silly strategies used in [tom7](http://tom7.org)'s fantastic video [30 Weird Chess Algorithms: Elo World](https://www.youtube.com/watch?v=DpXy041BIlA).

## Requirements

Because the engine uses [stockfish](https://stockfishchess.org/) for actual game engine decisioning, it must be installed and available in your `$PATH`.

This code should be portable, but I do not use Windows and am not confident it will work flawlessly.

## Execution

```
cargo run --release -- [options] <engine>
```

Run with `--help` for command-line usage. Current engines are:
* `random` (Choose moves at random)
* `pacifist` (Avoids making moves that checkmate, capture, or check)
* `stockfish` (Run Stockfish normally)
* `worstfish` (Use Stockfish to choose the worst-scoring moves)
* `mediocrefish` (Uses Stockfish to choose a median score move)
* `drawfish` (Uses Stockfish to attempt to stalemate)
* `scoville` (Plays X% of moves using Stockfish, diluting the rest with random moves)
