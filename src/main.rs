pub mod game;
pub mod players;
pub mod move_stack;

use indicatif::{ProgressBar, ProgressStyle};
use std::time::Instant;

use crate::players::{good_player::GoodPlayer, player::NewPlayerState, simple_player::SimplePlayer, player::{Player, PlayerState}};
use crate::game::{Game, SkipBoGame};

fn play_game(game: &mut SkipBoGame, players: Vec<Box<dyn Player>>) -> (i8, i32) {
    let mut n = 0;
    while !game.check_win() {
        for (index, player) in players.iter().enumerate() {
            game.play(index as i8, player);
        }

        n += 1;
    }

    (game.winner, n)
}

fn main() {
    let now = Instant::now();

    let mut wins: [i64; 2] = [0, 0];
    let mut rounds = 0;
    let rounds_to_play = 1000;

    let pb = ProgressBar::new(rounds_to_play as u64);
    pb.set_style(ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.green/blue}] {pos:>7}/{len:7} ({eta})"));
    for _ in 0..rounds_to_play {
        let mut game = SkipBoGame::new(vec![PlayerState::new(20), PlayerState::new(20)]);
        
        let (w, i) = play_game(&mut game, vec![Box::new(SimplePlayer {}), Box::new(GoodPlayer {})]);
        
        wins[w as usize] += 1;
        rounds += i;

        pb.inc(1);
    }
    pb.finish_and_clear();
    println!("Wins: {:?}, avg round length {}", wins, rounds / rounds_to_play);

    println!("Seconds elapsed: {}", now.elapsed().as_secs());
}
