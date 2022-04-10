pub mod player;
pub mod game;

use indicatif::{ProgressBar, ProgressStyle};

use player::{SimplePlayer, Player};
use crate::game::{Game, NewPlayerState, SkipBoGame, PlayerState};

fn play_game<T: Player, U: Player>(game: &mut SkipBoGame, player1: T, player2: U) -> (i8, i32) {
    let mut n = 0;
    while !game.check_win() {
        game.play(0, &player1);
        game.play(1, &player2);

        n += 1;
    }

    //println!("Player {} won the game in {} rounds!", game.winner, n);
    (game.winner, n)
}

fn main() {
    let mut wins: [i64; 2] = [0, 0];
    let mut rounds = 0;
    let rounds_to_play = 10000;

    let pb = ProgressBar::new(rounds_to_play as u64);
    pb.set_style(ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.green/blue}] {pos:>7}/{len:7} ({eta})"));
    for _ in 0..rounds_to_play {
        let mut game = SkipBoGame::new(vec![PlayerState::new(20), PlayerState::new(20)]);
        
        let (w, i) = play_game(&mut game, SimplePlayer {}, SimplePlayer {});
        
        wins[w as usize] += 1;
        rounds += i;

        pb.inc(1);
    }
    pb.finish_and_clear();
    println!("Wins: {:?}, avg round length {}", wins, rounds / rounds_to_play);
}
