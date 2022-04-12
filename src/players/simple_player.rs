use std::cmp::Ordering;

use crate::{players::player::Player, move_stack::{Move, CardStack}};

pub struct SimplePlayer {}
// Always plays the first valid_move (mostly from stack to playing_field) and puts the first card to the first SIDE stack
impl Player for SimplePlayer {
    fn select_move(&self, moves: Vec<Move>, _stack: i8, _opponent_stack: i8, _side: [Vec<i8>; 4], _hand: Vec<i8>, _playing_field: [(i8, bool); 4]) -> Option<Move> {
        let m = moves.first().unwrap();
        Some(Move { from: m.from, from_num: m.from_num, to: m.to, to_num: m.to_num })
    }

    fn select_stack(&self, _hand: Vec<i8>, side: [std::vec::Vec<i8>; 4]) -> Move {
        let index_of_min: usize = side
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.len().partial_cmp(&b.len()).unwrap_or(Ordering::Equal))
            .map(|(index, _)| index).unwrap();
        Move { from: CardStack::Hand, from_num: 0, to: CardStack::Side, to_num: index_of_min as i8 }
    }
}