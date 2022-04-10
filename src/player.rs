use std::cmp::Ordering;

use crate::game::{SkipBoGame, CardStack, Move, Game};

pub trait Player {
    fn play(&self, player_num: i8, game: &mut SkipBoGame) -> bool {
        let playing_field = game.playing_field;
        let p = &game.players[player_num as usize];

        let valid_moves = self.get_valid_moves(playing_field, p.hand.clone(), p.side.clone(), p.stack.clone());

        if valid_moves.is_empty() {
            // put card to side
            game.execute_move(player_num, &self.select_stack(game.players[player_num as usize].hand.clone(), game.players[player_num as usize].side.clone()));
            false
        } else {
            game.execute_move(player_num, &self.select_move(valid_moves));
            true
        }
    }

    fn select_move(&self, moves: Vec<Move>) -> Move;
    fn select_stack(&self, hand: Vec<i8>, side: [std::vec::Vec<i8>; 4]) -> Move;
    
    fn get_valid_moves(&self, playing_field: [i8; 4], hand: Vec<i8>, side: [Vec<i8>; 4], stack: Vec<i8>) -> Vec<Move> {
        let mut valid_moves = Vec::<Move>::new();

        // check if you can place any card from the stack
        for (index, f) in playing_field.iter().enumerate() {
            if stack.last().expect("empty stack") == f || (f == &12 && stack.last().expect("empty stack") == &1) {
                valid_moves.push(Move { from: CardStack::Stack, from_num: 0, to: CardStack::Field, to_num: index as i8 })
            }
        }

        // check if you can place any card from the hand
        for (index_f, f) in playing_field.iter().enumerate() {
            for (index_h, h) in hand.iter().enumerate() {
                if f+1 == *h || h == &-1 || (f == &12 && h == &1) {
                    valid_moves.push(Move { from: CardStack::Hand, from_num: index_h as i8, to: CardStack::Field, to_num: index_f as i8 })
                }
            }
        }

        // check if you can place any card from the side
        for (index_s, s) in side.iter().enumerate() {
            for (index_f, f) in playing_field.iter().enumerate() {
                if s.is_empty() {
                    continue;
                }
                if f+1 == *s.last().expect("empty stack") || s.last().expect("empty stack") == &-1 || (f == &12 && s.last().expect("empty stack") == &1) {
                    valid_moves.push(Move { from: CardStack::Side, from_num: index_s as i8, to: CardStack::Field, to_num: index_f as i8 })
                }
            }
        }

        valid_moves
    }
}

pub struct SimplePlayer {}
impl Player for SimplePlayer {
    fn select_move(&self, moves: Vec<Move>) -> Move {
        let m = moves.first().unwrap();
        Move { from: m.from, from_num: m.from_num, to: m.to, to_num: m.to_num }
    }

    fn select_stack(&self, hand: Vec<i8>, side: [std::vec::Vec<i8>; 4]) -> Move {
        let index_of_min: usize = side
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.len().partial_cmp(&b.len()).unwrap_or(Ordering::Equal))
            .map(|(index, _)| index).unwrap();
        Move { from: CardStack::Hand, from_num: 0, to: CardStack::Side, to_num: index_of_min as i8 }
    }
}