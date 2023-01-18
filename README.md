# Quantik

## Rules of the Game

_Taken from_ [here](https://boardgamegeek.com/boardgame/286295/quantik)

![](https://m.media-amazon.com/images/I/612j1vDPpsL._AC_SL1500_.jpg)

Quantik is a pure abstract strategy game. The goal is to be the first player to pose the fourth different forms of a line, a column or a square zone.

Each turn the players will put one of their pieces on the boardgame. It's forbidden to put a shape in a line, a column or an area on which this same form has already been posed by the opponent. We can only double a shape if we have played the previous one ourself. The first player who places the fourth different form in a row, column or zone wins the game immediately, no matter who owns the other pieces of that winning move.

## Playing in Rust

Instead of the shapes `Cylinder`, `Cone`, `Cube` and `Sphere`, we will use the letters `A`, `B`, `C` and `D`. Instead of colours, we will use upper case and lower case.

```
❯ cargo run
    Blocking waiting for file lock on build directory
   Compiling either v1.7.0
   Compiling itertools v0.10.3
   Compiling quantik v0.1.0 (/Users/dan5/Documents/coding/rust/quantik)
    Finished dev [unoptimized + debuginfo] target(s) in 1.93s
     Running `target/debug/quantik`
Round 0:
==================
 1 |  2 |  3 |  4
==================
 5 |  6 |  7 |  8
==================
 9 | 10 | 11 | 12
==================
13 | 14 | 15 | 16
==================
Player 0 has [Piece('A'), Piece('A'), Piece('B'), Piece('B'), Piece('C'), Piece('C'), Piece('D'), Piece('D')].
Player 0, select a piece: A
Player 0, choose a position: 1
Round 1:
==================
 A |  2 |  3 |  4
==================
 5 |  6 |  7 |  8
==================
 9 | 10 | 11 | 12
==================
13 | 14 | 15 | 16
==================
Player 1 has [Piece('a'), Piece('a'), Piece('b'), Piece('b'), Piece('c'), Piece('c'), Piece('d'), Piece('d')].
Player 1, select a piece: A
Player 1 does not have A
Player 1, select a piece: a
Player 1, choose a position: 6
[Error: Invalid placement at 6]
Player 1, select a piece: a
Player 1, choose a position: 10
Round 2:
==================
 A |  2 |  3 |  4
==================
 5 |  6 |  7 |  8
==================
 9 |  a | 11 | 12
==================
13 | 14 | 15 | 16
==================
Player 0 has [Piece('A'), Piece('B'), Piece('B'), Piece('C'), Piece('C'), Piece('D'), Piece('D')].
Player 0, select a piece: B
Player 0, choose a position: 2
Round 3:
==================
 A |  B |  3 |  4
==================
 5 |  6 |  7 |  8
==================
 9 |  a | 11 | 12
==================
13 | 14 | 15 | 16
==================
Player 1 has [Piece('a'), Piece('b'), Piece('b'), Piece('c'), Piece('c'), Piece('d'), Piece('d')].
Player 1, select a piece: b
Player 1, choose a position: 9
Round 4:
==================
 A |  B |  3 |  4
==================
 5 |  6 |  7 |  8
==================
 b |  a | 11 | 12
==================
13 | 14 | 15 | 16
==================
Player 0 has [Piece('A'), Piece('B'), Piece('C'), Piece('C'), Piece('D'), Piece('D')].
Player 0, select a piece: C
Player 0, choose a position: 3
Round 5:
==================
 A |  B |  C |  4
==================
 5 |  6 |  7 |  8
==================
 b |  a | 11 | 12
==================
13 | 14 | 15 | 16
==================
Player 1 has [Piece('a'), Piece('b'), Piece('c'), Piece('c'), Piece('d'), Piece('d')].
Player 1, select a piece: a
Player 1, choose a position: 15
Round 6:
==================
 A |  B |  C |  4
==================
 5 |  6 |  7 |  8
==================
 b |  a | 11 | 12
==================
13 | 14 |  a | 16
==================
Player 0 has [Piece('A'), Piece('B'), Piece('C'), Piece('D'), Piece('D')].
Player 0, select a piece: D
Player 0, choose a position: 4
==================
 A |  B |  C |  D
==================
 5 |  6 |  7 |  8
==================
 b |  a | 11 | 12
==================
13 | 14 |  a | 16
==================
Player 0 is the winner!
======== THE END ========
```

The UI will need some improvements but the functionality is there.
