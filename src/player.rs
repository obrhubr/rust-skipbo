use std::cmp::Ordering;

use crate::game::{SkipBoGame, CardStack, Move, Game};

fn distance_between_cards(c1: i8, c2: i8) -> i8 {
    if c2 < c1 {
        12 - c1 + c2
    } else {
        c2 - c1
    }
}

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
            let selected_move = self.select_move(valid_moves, *game.players[player_num as usize].stack.last().unwrap(), *game.players[((player_num + 1) % (game.players.len() as i8)) as usize].stack.last().unwrap(), game.players[player_num as usize].side.clone(), game.players[player_num as usize].hand.clone(), game.playing_field);
            match selected_move {
                None => {
                    // Put card to side if player executes no move and end turn
                    game.execute_move(player_num, &self.select_stack(game.players[player_num as usize].hand.clone(), game.players[player_num as usize].side.clone()));
                    return false
                }
                Some(m) => {
                    game.execute_move(player_num, &m);
                }
            }
            true
        }
    }

    fn select_move(&self, moves: Vec<Move>, stack: i8, opponent_stack: i8, side: [Vec<i8>; 4], hand: Vec<i8>, playing_field: [i8; 4]) -> Option<Move>;
    fn select_stack(&self, hand: Vec<i8>, side: [std::vec::Vec<i8>; 4]) -> Move;
    
    fn get_valid_moves(&self, playing_field: [i8; 4], hand: Vec<i8>, side: [Vec<i8>; 4], stack: Vec<i8>) -> Vec<Move> {
        let mut valid_moves = Vec::<Move>::new();

        // check if you can place any card from the stack
        for (index, f) in playing_field.iter().enumerate() {
            let l = stack.last().expect("empty stack");
            if l == &(f+1) || (f == &12 && l == &1) || l == &-1 {
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
                let l = s.last().expect("empty stack");
                if f+1 == *l || l == &-1 || (f == &12 && l == &1) {
                    valid_moves.push(Move { from: CardStack::Side, from_num: index_s as i8, to: CardStack::Field, to_num: index_f as i8 })
                }
            }
        }

        valid_moves
    }
}

pub struct SimplePlayer {}
// Always plays the first valid_move (mostly from stack to playing_field)
impl Player for SimplePlayer {
    fn select_move(&self, moves: Vec<Move>, _stack: i8, _opponent_stack: i8, _side: [Vec<i8>; 4], _hand: Vec<i8>, _playing_field: [i8; 4]) -> Option<Move> {
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

pub struct BadPlayer {}
// Only plays from stack
impl Player for BadPlayer {
    fn select_move(&self, moves: Vec<Move>, _stack: i8, _opponent_stack: i8, _side: [Vec<i8>; 4], _hand: Vec<i8>, _playing_field: [i8; 4]) -> Option<Move> {
        let m = moves.first().unwrap();
        if m.from == CardStack::Stack {
            Some(Move { from: m.from, from_num: m.from_num, to: m.to, to_num: m.to_num })
        } else {
            None
        }
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


trait RecursivePlayer {
    fn recurse_stack (&self, stack: i8, playing_field: i8, playing_field_stack: i8, hand: Vec<i8>, side: [Vec<i8>; 4]) -> Option<Move>;
}

pub struct GoodPlayer {}
impl RecursivePlayer for GoodPlayer {
    fn recurse_stack (&self, stack: i8, playing_field: i8, playing_field_stack: i8, mut hand: Vec<i8>, mut side: [Vec<i8>; 4]) -> Option<Move> {
        // Is any card from HAND a card that could come before playing_field
        match hand.iter().position(|c| c == &(stack - 1)) {
            None => {}
            Some(card) => {
                // remove that card from hand
                hand.remove(card);
                // recurse
                if (stack-2) == playing_field {
                    return Some(Move { to: CardStack::Field, to_num: playing_field_stack, from: CardStack::Hand, from_num: card as i8 });
                } else {
                    return self.recurse_stack(stack - 1, playing_field, playing_field_stack, hand, side);
                }
            }
        }

        match hand.iter().position(|c| c == &-1) {
            None => {}
            Some(card) => {
                // remove that card from hand
                hand.remove(card);
                // recurse
                if (stack-2) == playing_field {
                    return Some(Move { to: CardStack::Field, to_num: playing_field_stack, from: CardStack::Hand, from_num: card as i8 });
                } else {
                    return self.recurse_stack(stack - 1, playing_field, playing_field_stack, hand, side);
                }
            }
        }

        // Is any card from SIDE a card that could come before playing_field
        for (index, side_stack) in side.iter_mut().enumerate() {
            match side_stack.last() {
                None => {}
                Some(s) => {
                    if *s == (stack - 1) || *s == -1 {
                        // remove that card from side
                        side_stack.pop().unwrap();
                        // recurse
                        if (stack-2) == playing_field {
                            return Some(Move { to: CardStack::Field, to_num: playing_field_stack, from: CardStack::Side, from_num: index as i8 });
                        } else {
                            return self.recurse_stack(stack - 1, playing_field, playing_field_stack, hand, side);
                        }
                    }
                }
            };
        }

        None
    }
}
impl Player for GoodPlayer {
    
    fn select_move(&self, moves: Vec<Move>, stack: i8, opponent_stack: i8, side: [Vec<i8>; 4], hand: Vec<i8>, playing_field: [i8; 4]) -> Option<Move> {
        // Play cards from STACK
        let first_move = moves.first().unwrap();
        if first_move.from == CardStack::Stack {
            return Some(Move { from: first_move.from, from_num: first_move.from_num, to: first_move.to, to_num: first_move.to_num });
        }

        // Play cards from HAND & SIDE if you can play a card from STACK
        // recursively iterate through the cards in hand and side from the current stack card
        for (index, p) in playing_field.iter().enumerate() {
            let m = self.recurse_stack(stack, *p, index as i8, hand.clone(), side.clone());
            if m.is_some() { 
                return m
            };
        }

        // Play cards from HAND & SIDE if you can prevent the next player from playing a card from stack
        for (index, p) in playing_field.iter().enumerate() {
            let m = self.recurse_stack(opponent_stack+1, *p, index as i8, hand.clone(), side.clone());
            if m.is_some() { 
                return m
            };
        }


        // Play cards from HAND that are not (-1) or (cards whose values are less than STACK - 3)
        let vc: Vec<&Move> = moves.iter().filter(|m| m.from == CardStack::Hand).filter(|m| distance_between_cards(hand[m.from_num as usize], opponent_stack) > 3).collect();
        if !vc.is_empty() {
            for fm in vc {
                if hand[fm.from_num as usize] != -1 {
                    return Some(Move { from: fm.from, from_num: fm.from_num, to: fm.to, to_num: fm.to_num });
                }
            }
        }

        None
    }

    fn select_stack(&self, hand: Vec<i8>, side: [std::vec::Vec<i8>; 4]) -> Move {
        // Play card that is already on a SIDE
        for (index_s, s) in side.iter().enumerate() {
            match s.last() {    
                None => {}
                Some(side_card) => {
                    for (index_h, h) in hand.iter().enumerate() {
                        if side_card == h {
                            return Move { to: CardStack::Side, to_num: index_s as i8, from: CardStack::Hand, from_num: index_h as i8 }
                        }
                    }
                }
            }
        }

        // Play card that is multiple times in HAND to any free side
        let duplicate = (1..hand.len()).position(|i| hand[i..].contains(&hand[i - 1]));
        let empty_side = side.iter().position(|s| s.is_empty());
        match duplicate {
            None => {}
            Some(d) => {
                match empty_side {
                    None => {}
                    Some(e) => {
                        return Move { to: CardStack::Side, to_num: e as i8, from: CardStack::Hand, from_num: d as i8};
                    }
                }
            }
        }


        // Play any (except (-1)) card to any free SIDE
        match empty_side {
            None => {   
                // Play cards of value  (SIDE - 1)
                for (index_h, h) in hand.iter().enumerate() {
                    for (index_s, s) in side.iter().enumerate() {
                        match s.last() {
                            None => {}
                            Some(side_stack) => {
                                if h+1 == *side_stack {
                                    return Move { from: CardStack::Hand, from_num: index_h as i8, to: CardStack::Side, to_num: index_s as i8 };
                                }
                            }
                        }
                    }
                }
            }
            
            Some(e) => {
                return Move { from: CardStack::Hand, from_num: 0, to: CardStack::Side, to_num: e as i8 };
            }
        }


        // Play any card to SIDE 1
        Move { from: CardStack::Hand, from_num: 0, to: CardStack::Side, to_num: 0 }
    }
}