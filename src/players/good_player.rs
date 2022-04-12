use crate::{players::player::Player, move_stack::{Move, CardStack}};

fn distance_between_cards(c1: i8, c2: i8) -> i8 {
    if c2 < c1 {
        12 - c1 + c2
    } else {
        c2 - c1
    }
}

trait RecursivePlayer {
    fn recurse_stack (&self, stack: i8, playing_field: (i8, bool), playing_field_stack: i8, hand: Vec<i8>, fixed_hand: Vec<i8>, side: [Vec<i8>; 4], fixed_side: [Vec<i8>; 4], used_stack: bool, used_joker: bool) -> Option<Move>;
}

pub struct GoodPlayer {}
impl RecursivePlayer for GoodPlayer {
    fn recurse_stack (&self, stack: i8, playing_field: (i8, bool), playing_field_stack: i8, mut hand: Vec<i8>, fixed_hand: Vec<i8>, mut side: [Vec<i8>; 4], fixed_side: [Vec<i8>; 4], used_stack: bool, used_joker: bool) -> Option<Move> {
        // Is any card from HAND a card that could come before playing_field
        match hand.iter().position(|c| c == &(stack - 1)) {
            None => {}
            Some(card) => {
                // recurse
                if (stack-2) == playing_field.0 {
                    match fixed_hand.iter().position(|c| *c == hand[card]) {
                        None => {},
                        Some(card_pos) => {  
                            return Some(Move { to: CardStack::Field, to_num: playing_field_stack, from: CardStack::Hand, from_num: card_pos as i8 });
                        }
                    }
                } else {
                    // remove that card from hand
                    hand.remove(card);
                    return self.recurse_stack(stack - 1, playing_field, playing_field_stack, hand, fixed_hand, side, fixed_side, used_stack, false);
                }
            }
        }

        // Is any card from SIDE a card that could come before playing_field
        if !used_stack {
            for (index, side_stack) in side.iter_mut().enumerate() {
                match side_stack.last() {
                    None => {}
                    Some(s) => {
                        if *s == (stack - 1) || *s == -1 {
                            // recurse
                            if (stack-2) == playing_field.0 && ((s == &-1) ^ playing_field.1) {
                                return Some(Move { to: CardStack::Field, to_num: playing_field_stack, from: CardStack::Side, from_num: index as i8 });
                            } else {
                                // remove that card from side
                                side_stack.pop().unwrap();
                                return self.recurse_stack(stack - 1, playing_field, playing_field_stack, hand, fixed_hand, side, fixed_side, true, false);
                            }
                        }
                    }
                };
            }
        }

        // Is there any joker in HAND?
        if !used_joker {
            match hand.iter().position(|c| c == &-1) {
                None => {}
                Some(card) => {
                    // recurse
                    if (stack-2) == playing_field.0 && !playing_field.1 {
                        match fixed_hand.iter().position(|c| *c == hand[card]) {
                            None => {},
                            Some(card_pos) => {  
                                return Some(Move { to: CardStack::Field, to_num: playing_field_stack, from: CardStack::Hand, from_num: card_pos as i8 });
                            }
                        }
                    } else {
                        // remove that card from hand
                        hand.remove(card);
                        return self.recurse_stack(stack - 1, playing_field, playing_field_stack, hand, fixed_hand, side, fixed_side, used_stack, true);
                    }
                }
            }
        }

        None
    }
}

impl Player for GoodPlayer {
    fn select_move(&self, moves: Vec<Move>, stack: i8, opponent_stack: i8, side: [Vec<i8>; 4], hand: Vec<i8>, playing_field: [(i8, bool); 4]) -> Option<Move> {
        // Play cards from STACK
        let first_move = moves.first().unwrap();
        if first_move.from == CardStack::Stack {
            return Some(Move { from: first_move.from, from_num: first_move.from_num, to: first_move.to, to_num: first_move.to_num });
        }

        // Play cards from HAND & SIDE if you can play a card from STACK
        // recursively iterate through the cards in hand and side from the current stack card
        for (index, p) in playing_field.iter().enumerate() {
            let m = self.recurse_stack(stack, *p, index as i8, hand.clone(), hand.clone(), side.clone(), side.clone(), false, false);
            if m.is_some() {
                return m
            };
        }

        // Play cards from HAND & SIDE if you can prevent the next player from playing a card from stack
        for (index, p) in playing_field.iter().enumerate() {
            let m = self.recurse_stack(opponent_stack+1, *p, index as i8, hand.clone(), hand.clone(), side.clone(), side.clone(), false, false);
            if m.is_some() {
                return m
            };
        }


        // Play cards from HAND that are not (-1) or (cards whose values are less than OPPONENT_STACK - 3)
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