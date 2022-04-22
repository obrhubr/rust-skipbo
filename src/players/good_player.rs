use crate::{players::player::Player, move_stack::{Move, CardStack}};

// Calculate the amount of cards needed to go from one card to another
fn distance_between_cards(c1: i8, c2: i8) -> i8 {
    if c2 < c1 {
        12 - c1 + c2
    } else {
        c2 - c1
    }
}

trait RecursivePlayer {
    fn recurse_stack (
        &self, 
        stack: i8, 
        playing_field: (i8, bool), 
        playing_field_stack: i8, 
        hand: Vec<i8>, 
        fixed_hand: Vec<i8>, 
        side: [Vec<i8>; 4], 
        fixed_side: [Vec<i8>; 4], 
        used_stack: bool, 
        used_joker: bool
    ) -> Option<Move>;
}

pub struct GoodPlayer {}
impl RecursivePlayer for GoodPlayer {
    // TODO: Recurse feed forward
    // Recursively check if the player could play a card from STACK by using the other availiable cards
    fn recurse_stack (&self, stack: i8, playing_field: (i8, bool), playing_field_stack: i8, mut hand: Vec<i8>, fixed_hand: Vec<i8>, mut side: [Vec<i8>; 4], fixed_side: [Vec<i8>; 4], used_stack: bool, used_joker: bool) -> Option<Move> {
        // Is any card from HAND a card that could come before stack
        match hand.iter().position(|c| c == &(stack - 1)) {
            None => {}
            Some(card) => {
                // If the player has all necessary cards to reach his STACK, return the first move
                if (stack-2) == playing_field.0 {
                    match fixed_hand.iter().position(|c| *c == hand[card]) {
                        None => {},
                        Some(card_pos) => {  
                            return Some(Move { to: CardStack::Field, to_num: playing_field_stack, from: CardStack::Hand, from_num: card_pos as i8 });
                        }
                    }
                } else {
                    // Remove card from hand
                    hand.remove(card);

                    // If the player has the card, check if he also has a card that comes before that card (lower the value of stack to that of the current card)
                    return self.recurse_stack(stack - 1, playing_field, playing_field_stack, hand, fixed_hand, side, fixed_side, used_stack, false);
                }
            }
        }

        // Is any card from SIDE a card that could come before stack
        if !used_stack {
            for (index, side_stack) in side.iter_mut().enumerate() {
                match side_stack.last() {
                    None => {}
                    Some(s) => {
                        if *s == (stack - 1) || *s == -1 {
                            // If the player has all necessary cards to reach his STACK, return the first move
                            if (stack-2) == playing_field.0 && /* If the card is a joker, check if joker is allowed */((s == &-1) ^ playing_field.1) {
                                return Some(Move { to: CardStack::Field, to_num: playing_field_stack, from: CardStack::Side, from_num: index as i8 });
                            } else {
                                // remove that card from side
                                side_stack.pop().unwrap();

                                // If the player has the card, check if he also has a card that comes before that card (lower the value of stack to that of the current card)
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
                    // If the player has all necessary cards to reach his STACK, return the first move
                    if (stack-2) == playing_field.0 && !playing_field.1 {
                        match fixed_hand.iter().position(|c| *c == hand[card]) {
                            None => {},
                            Some(card_pos) => {  
                                // If the player has all necessary cards to reach his STACK, return the first move
                                return Some(Move { to: CardStack::Field, to_num: playing_field_stack, from: CardStack::Hand, from_num: card_pos as i8 });
                            }
                        }
                    } else {
                        // remove that card from hand
                        hand.remove(card);
                        
                        // If the player has the card, check if he also has a card that comes before that card (lower the value of stack to that of the current card)
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
        // If the player can play any card from STACK, do so
        let first_move = moves.first().unwrap();
        if first_move.from == CardStack::Stack {
            return Some(*first_move);
        }

        // Play cards from HAND & SIDE if the player can play a card from STACK
        for (index, p) in playing_field.iter().enumerate() {
            // recursively iterate through the cards in hand and side
            let m = self.recurse_stack(stack, *p, index as i8, hand.clone(), hand.clone(), side.clone(), side.clone(), false, false);
            if m.is_some() {
                return m
            };
        }

        // Play cards from HAND & SIDE if you can prevent the next player from playing a card from stack (max 3 cards played)
        for (index, p) in playing_field.iter().enumerate() {
            // recursively iterate through the cards in hand and side
            if distance_between_cards(p.0, opponent_stack) < 4 {
                let m = self.recurse_stack(opponent_stack+1, *p, index as i8, hand.clone(), hand.clone(), side.clone(), side.clone(), false, false);
                if m.is_some() {
                    return m
                };
            }
        }


        // Play cards from HAND that are not jokers or cards whose values are less than OPPONENT_STACK - 3 (they would help the opponent)
        let vc: Vec<&Move> = moves.iter().filter(|m| m.from == CardStack::Hand).filter(|m| distance_between_cards(hand[m.from_num as usize], opponent_stack) > 3).collect();
        if !vc.is_empty() {
            for m in vc {
                // Check if card is joker
                if hand[m.from_num as usize] != -1 {
                    return Some(*m);
                }
            }
        }

        // Play cards from SIDE that are not jokers or cards whose values are less than OPPONENT_STACK - 3 (otherwise they would help the opponent)
        let vc: Vec<&Move> = moves.iter().filter(|m| m.from == CardStack::Side).filter(|m| distance_between_cards(*side[m.from_num as usize].last().unwrap(), opponent_stack) > 3).collect();
        if !vc.is_empty() {
            for m in vc {
                // Check if card is joker
                if *side[m.from_num as usize].last().unwrap() != -1 {
                    return Some(*m);
                }
            }
        }

        // No move corresponds to the criteria and the player chooses not to play any card
        None
    }

    fn select_stack(&self, hand: Vec<i8>, side: [std::vec::Vec<i8>; 4]) -> Move {
        // Play card to stack that already has that card
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

        // Play card that the player has multiple times in HAND to any free SIDE
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


        // Play any card (except joker) card to any free SIDE
        match empty_side {
            // If there is no free SIDE, try to play card to stack that has a card that has the value (card + 1) ex: play 8 to a SIDE with 9 on top, so you can play 8 and then 9
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


        // Play any card to the first SIDE stack
        Move { from: CardStack::Hand, from_num: 0, to: CardStack::Side, to_num: 0 }
    }
}