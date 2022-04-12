use crate::player::Player;
use rand::{Rng, prelude::ThreadRng};

#[derive(std::cmp::PartialEq, Debug, Copy, Clone)]
pub enum CardStack {
    Stack,
    Side,
    Hand,
    Field
}

#[derive(Debug, std::cmp::PartialEq, Clone, Copy)]
pub struct Move {
    pub from: CardStack,
    pub from_num: i8,
    pub to: CardStack,
    pub to_num: i8
}

pub trait Game {
    fn new(players: Vec<PlayerState>) -> Self;

    fn to_playing_field_from_stack(&mut self, player_num: i8, stack: i8);
    fn to_playing_field_from_hand(&mut self, player_num: i8, stack_field: i8, stack_hand: i8);
    fn to_playing_field_from_side(&mut self, player_num: i8, stack_field: i8, stack_side: i8);
    fn to_side(&mut self, player_num: i8, card: i8, stack: i8);
    fn execute_move(&mut self, player_num: i8, m: &Move);
    fn refill_hand(&mut self, player_num: i8);

    fn check_win(&mut self) -> bool;

    fn play(&mut self, player_num: i8, player: &Box<dyn Player>);
}

pub trait NewPlayerState {
    fn new(stack_size: i32) -> Self;
}

pub struct PlayerState {
    pub hand: Vec<i8>,
    pub side: [Vec<i8>; 4],
    pub stack: Vec<i8>
}

impl NewPlayerState for PlayerState {
    fn new(stack_size: i32) -> Self {
        let mut rng = rand::thread_rng();

        let mut stack: Vec<i8> = Vec::new();
        for _ in 0..stack_size {
            let mut random_num = rng.gen_range(1..14);
            if random_num == 13 { random_num = -1 };

            stack.push(random_num);
        }

        PlayerState { 
            hand: vec![rng.gen_range(1..13), rng.gen_range(1..13), rng.gen_range(1..13), rng.gen_range(1..13), rng.gen_range(1..13)], 
            side: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            stack
        }
    }
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

    fn play(&mut self, player_num: i8, player: &Box<dyn Player>) {
        self.refill_hand(player_num);

        let mut finished = true;
        while finished {
            if self.check_win() {
                break;
            }

            finished = player.play(player_num, self);

            if self.players[player_num as usize].hand.is_empty() && finished {
                self.refill_hand(player_num);
            }
        }
    }
}