pub mod game;
pub mod players;
pub mod move_stack;

use indicatif::{ProgressBar, ProgressStyle};
use std::{time::Instant, vec, ffi::FromVecWithNulError};

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

fn play_n_games(player_number: i64, n: i32) -> Vec<i64> {
    let mut wins: Vec<i64> = vec![0; player_number as usize];
    let mut rounds = 0;
    let rounds_to_play = n;

    /* let pb = ProgressBar::new(rounds_to_play as u64);
    pb.set_style(ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.green/blue}] {pos:>7}/{len:7} ({eta})")); */
    for _ in 0..rounds_to_play {
        let mut game = SkipBoGame::new(vec![PlayerState::new(20), PlayerState::new(20)]);
        
        let (w, i) = play_game(&mut game, vec![Box::new(SimplePlayer {}), Box::new(GoodPlayer {})]);
        
        wins[w as usize] += 1;
        rounds += i;

        /* pb.inc(1); */
    }
    /* pb.finish_and_clear(); */
    //println!("Wins: {:?}, avg round length {}", wins, rounds / rounds_to_play);

    wins
}

fn calc_stats(mut winrates: Vec<i64>) -> (f64, i64, i64, f64, i64, f64) {
    winrates.sort_unstable();

    // Convert to distribution
    let lowest = winrates.first().unwrap(); 
    let highest = winrates.last().unwrap();
    let dist_range = highest - lowest;

    let avg = (winrates.iter().sum::<i64>() as f64) / (winrates.len() as f64);

    let mut lower: Vec<i64> = Vec::new();
    for v in winrates.clone() {
        if v > (avg as i64) {
            break;
        } else {
            lower.push(v);
        }
    }
    let avg_lower = (lower.iter().sum::<i64>() as f64) / (lower.len() as f64);

    let mut higher: Vec<i64> = Vec::new();
    for v in winrates.clone() {
        if v < (avg as i64) {
            continue;
        } else {
            higher.push(v);
        }
    }
    let avg_higher = (higher.iter().sum::<i64>() as f64) / (higher.len() as f64);

    (avg, dist_range, *lowest, avg_lower, *highest, avg_higher)
}

fn get_stats() -> (i64, Vec<(f64, i64, i64, f64, i64, f64)>) {
    let player_num: i64 = 2;
    let mut winrates: Vec<Vec<i64>> = vec![Vec::new(); player_num as usize];
    let games: i64 = 100;
    let rounds: i64 = 2000;

    let pb = ProgressBar::new(rounds as u64);
    pb.set_style(ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.green/blue}] {pos:>7}/{len:7} ({eta})"));
    for _ in 1..rounds {
        let wins = play_n_games(player_num, games as i32);
        // get winrate of goodplayer
        for (i, w) in wins.iter().enumerate() {
            winrates[i].push(*w);
        }

        pb.inc(1);
    }
    pb.finish_and_clear();

    let mut stats: Vec<(f64, i64, i64, f64, i64, f64)> = Vec::new();
    for w in winrates {
        stats.push(calc_stats(w));
    }

    (games * rounds, stats)
}

fn main() {
    let now = Instant::now();

    let (n, stats) = get_stats();

    println!("Games played: {}\n", n);

    for (i, s) in stats.iter().enumerate() {
        println!("PlayerNum: {} \nAvg: {:.2} \nRange: {} \nLowest: {}  |  Lower Avg: {:.2} \nHighest: {} |  Higher Avg: {:.2}\n", i, s.0, s.1, s.2, s.3, s.4, s.5);
    }

    println!("Seconds elapsed: {}", now.elapsed().as_secs());
}
