use crate::{players::player::{Player, PlayerState}, move_stack::{Move, CardStack}};
use rand::{Rng, prelude::ThreadRng};

pub trait Game {
    fn new(players: Vec<PlayerState>) -> Self;

    fn to_playing_field_from_stack(&mut self, player_num: i8, stack: i8);
    fn to_playing_field_from_hand(&mut self, player_num: i8, stack_field: i8, stack_hand: i8);
    fn to_playing_field_from_side(&mut self, player_num: i8, stack_field: i8, stack_side: i8);
    fn to_side(&mut self, player_num: i8, card: i8, stack: i8);
    fn execute_move(&mut self, player_num: i8, m: &Move);
    fn refill_hand(&mut self, player_num: i8);

    fn check_win(&mut self) -> bool;

    fn get_valid_moves(&self, playing_field: [(i8, bool); 4], hand: Vec<i8>, side: [Vec<i8>; 4], stack: Vec<i8>) -> Vec<Move>;
    fn turn(&mut self, player_num: i8, player: &Box<dyn Player>) -> bool;
    fn play(&mut self, player_num: i8, player: &Box<dyn Player>);
}

pub struct SkipBoGame {
    pub playing_field: [(i8, bool); 4],
    pub players: Vec<PlayerState>,
    pub end: bool,
    pub winner: i8,
    pub rng: ThreadRng
}

impl Game for SkipBoGame {
    fn new(players: Vec<PlayerState>) -> Self {
        SkipBoGame {
            playing_field: [(12, false), (12, false), (12, false), (12, false)], 
            players,
            end: false,
            winner: -1,
            rng: rand::thread_rng()
        }
    }

    fn to_playing_field_from_stack(&mut self, player_num: i8, stack: i8) {
        let card = self.players[player_num as usize].stack.last().expect("stack is not empty");

        // Set card as playing_field (if it is joker, replace by next)
        if card == &-1 {
            if self.playing_field[stack as usize].1 {
                println!("ERROR");
            }
            if self.playing_field[stack as usize].0 == 12 {
                self.playing_field[stack as usize] = (1, true);
            } else {
                self.playing_field[stack as usize].0 += 1;
                self.playing_field[stack as usize].1 = true;
            }
        } else {
            self.playing_field[stack as usize] = (*card, false);
        }
        // Remove card from player's stack
        self.players[player_num as usize].stack.pop().expect("Stack not emtpy");
    }

    fn to_playing_field_from_hand(&mut self, player_num: i8, stack_field: i8, stack_hand: i8) {
        let card = self.players[player_num as usize].hand[stack_hand as usize];
        
        // Set card as playing_field (if it is joker, replace by next)
        if card == -1 {
            if self.playing_field[stack_field as usize].1 {
                println!("ERROR");
            }
            if self.playing_field[stack_field as usize].0 == 12 {
                self.playing_field[stack_field as usize] = (1, true);
            } else {
                self.playing_field[stack_field as usize].0 += 1;
                self.playing_field[stack_field as usize].1 = true;
            }
        } else {
            self.playing_field[stack_field as usize] = (card, false);
        }
        // Remove card from player's hand
        self.players[player_num as usize].hand.remove(stack_hand as usize);
    }

    fn to_playing_field_from_side(&mut self, player_num: i8, stack_field: i8, stack_side: i8) {
        let card = self.players[player_num as usize].side[stack_side as usize].last().copied().expect("Stack contains at least one card and it is copyable.");
        
        // Set card as playing_field (if it is joker, replace by next)
        if card == -1 {
            if self.playing_field[stack_field as usize].1 {
                println!("ERROR");
            }
            if self.playing_field[stack_field as usize].0 == 12 {
                self.playing_field[stack_field as usize] = (1, true);
            } else {
                self.playing_field[stack_field as usize].0 += 1;
                self.playing_field[stack_field as usize].1 = true;
            }
        } else {
            self.playing_field[stack_field as usize] = (card, false);
        }
        // Remove card from player's side
        self.players[player_num as usize].side[stack_side as usize].pop();
    }

    fn to_side(&mut self, player_num: i8, stack_hand: i8, stack_side: i8) {
        let card = self.players[player_num as usize].hand[stack_hand as usize];
        self.players[player_num as usize].side[stack_side as usize].push(card);
        self.players[player_num as usize].hand.remove(stack_hand as usize);
    }

    fn execute_move(&mut self, player_num: i8, m: &Move) {
        if m.from == CardStack::Stack {
            self.to_playing_field_from_stack(player_num, m.to_num);
        }

        if m.from == CardStack::Hand {
            if m.to == CardStack::Field {
                self.to_playing_field_from_hand(player_num, m.to_num, m.from_num);
            } else if m.to == CardStack::Side {
                self.to_side(player_num, m.from_num, m.to_num);
            }
        }

        if m.from == CardStack::Side {
            self.to_playing_field_from_side(player_num, m.to_num, m.from_num)
        }
    }

    fn refill_hand(&mut self, player_num: i8) {
        let length = self.players[player_num as usize].hand.len();

        if length < 5 {
            let mut new_cards: Vec<i8> = Vec::new();
            for _ in 0..(5 - length) {
                let mut random_num = self.rng.gen_range(1..14);
                if random_num == 13 { random_num = -1 };
                new_cards.push(random_num);
            }  
            self.players[player_num as usize].hand.append(&mut new_cards);
        }
    }

    fn check_win(&mut self) -> bool {
        for (index, player) in self.players.iter().enumerate() {
            if player.stack.is_empty() {
                self.end = true;
                self.winner = index as i8;
                return true
            }
        }
        false
    }

    fn get_valid_moves(&self, playing_field: [(i8, bool); 4], hand: Vec<i8>, side: [Vec<i8>; 4], stack: Vec<i8>) -> Vec<Move> {
        let mut valid_moves = Vec::<Move>::new();

        // check if you can place any card from the stack
        for (index, f) in playing_field.iter().enumerate() {
            let l = stack.last().expect("empty stack");
            if l == &(f.0 + 1) || (f.0 == 12 && l == &1) || (l == &-1 && !f.1) {
                valid_moves.push(Move { from: CardStack::Stack, from_num: 0, to: CardStack::Field, to_num: index as i8 })
            }
        }

        // check if you can place any card from the hand
        for (index_f, f) in playing_field.iter().enumerate() {
            for (index_h, h) in hand.iter().enumerate() {
                if f.0 + 1 == *h || (h == &-1 && !f.1) || (f.0 == 12 && h == &1) {
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
                if f.0 +1 == *l || (l == &-1 && !f.1) || (f.0 == 12 && l == &1) {
                    valid_moves.push(Move { from: CardStack::Side, from_num: index_s as i8, to: CardStack::Field, to_num: index_f as i8 })
                }
            }
        }

        valid_moves
    }

    fn turn(&mut self, player_num: i8, player: &Box<dyn Player>) -> bool {
        let playing_field = self.playing_field;
        let p = &self.players[player_num as usize];

        let valid_moves = self.get_valid_moves(playing_field, p.hand.clone(), p.side.clone(), p.stack.clone());

        if valid_moves.is_empty() {
            // put card to side
            self.execute_move(player_num, &player.select_stack(self.players[player_num as usize].hand.clone(), self.players[player_num as usize].side.clone()));
            false
        } else {
            let selected_move = player.select_move(valid_moves, *self.players[player_num as usize].stack.last().unwrap(), *self.players[((player_num + 1) % (self.players.len() as i8)) as usize].stack.last().unwrap(), self.players[player_num as usize].side.clone(), self.players[player_num as usize].hand.clone(), self.playing_field);
            match selected_move {
                None => {
                    // Put card to side if player executes no move and end turn
                    self.execute_move(player_num, &player.select_stack(self.players[player_num as usize].hand.clone(), self.players[player_num as usize].side.clone()));
                    return false
                }
                Some(m) => {
                    self.execute_move(player_num, &m);
                }
            }
            true
        }
    }

    fn play(&mut self, player_num: i8, player: &Box<dyn Player>) {
        self.refill_hand(player_num);

        let mut finished = true;
        while finished {
            if self.check_win() {
                break;
            }

            finished = self.turn(player_num, player);

            if self.players[player_num as usize].hand.is_empty() && finished {
                self.refill_hand(player_num);
            }
        }
    }
}