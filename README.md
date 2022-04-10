# Rust Skipbo

### Rules

![Skipbo](./.github/skipbo.jpg)

The rules can be read at [https://en.wikipedia.org/wiki/Skip-Bo](https://en.wikipedia.org/wiki/Skip-Bo).
My implementation of the game assumes 12 different cards and 1 joker.

### Implementation

Each round, a `Player` object is passed to the `play()` method invoked on the `Game` object. Inside the `play` function the gamestate is handed over to the `play()` method of the `Player` object. The `Player` then plays as many cards as he wants and then puts one of his cards onto the "side"-stack.

To make a move (play a card) the `Player` passes a `Move` struct which contains the `to`-stack  and `from`-stack to the `execute_move()` method on the `Game` object. To choose a move, the `Player` calls the `get_valid_moves()` method which returns a `Vec` of moves.

If you want your player to follow a specific strategy, you can create implement your own `Player` object and change how he selects the move and the card to set aside.

### Getting Started

Run: `cargo run` and watch as the games are played.