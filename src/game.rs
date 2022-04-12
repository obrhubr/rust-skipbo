use crate::{players::player::{Player, PlayerState}, move_stack::{Move, CardStack}};
use rand::{Rng, prelude::ThreadRng};

pub trait Game {
    fn new(players: Vec<PlayerState>) -> Self;

    // Modifying game state
    fn to_playing_field_from_stack(&mut self, player_num: i8, stack: i8);
    fn to_playing_field_from_hand(&mut self, player_num: i8, stack_field: i8, stack_hand: i8);
    fn to_playing_field_from_side(&mut self, player_num: i8, stack_field: i8, stack_side: i8);
    fn to_side(&mut self, player_num: i8, card: i8, stack: i8);
    fn execute_move(&mut self, player_num: i8, m: &Move);
    fn refill_hand(&mut self, player_num: i8);

    // Playing the game
    fn get_valid_moves(&self, playing_field: [(i8, bool); 4], hand: Vec<i8>, side: [Vec<i8>; 4], stack: Vec<i8>) -> Vec<Move>;
    fn play_move(&mut self, player_num: i8, player: &Box<dyn Player>) -> bool;
    fn play(&mut self, player_num: i8, player: &Box<dyn Player>);
    fn check_win(&mut self) -> bool;
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

    // Modify gamestate to move card from STACK to FIELD
    fn to_playing_field_from_stack(&mut self, player_num: i8, stack: i8) {
        let card = self.players[player_num as usize].stack.last().expect("stack is not empty");

        if card == &-1 {
            // if the card is a joker, set the "joker-flag" to true
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

    // Modify gamestate to move card from HAND to FIELD
    fn to_playing_field_from_hand(&mut self, player_num: i8, stack_field: i8, stack_hand: i8) {
        let card = self.players[player_num as usize].hand[stack_hand as usize];
        
        // if the card is a joker, set the "joker-flag" to true
        if card == -1 {
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
    
    // Modify gamestate to move card from SIDE to FIELD
    fn to_playing_field_from_side(&mut self, player_num: i8, stack_field: i8, stack_side: i8) {
        let card = self.players[player_num as usize].side[stack_side as usize].last().copied().expect("Stack contains at least one card and it is copyable.");
        
        // if the card is a joker, set the "joker-flag" to true
        if card == -1 {
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
    
    // Modify gamestate to move card from HAND to SIDE (called after every turn)
    fn to_side(&mut self, player_num: i8, stack_hand: i8, stack_side: i8) {
        let card = self.players[player_num as usize].hand[stack_hand as usize];
        self.players[player_num as usize].side[stack_side as usize].push(card);
        self.players[player_num as usize].hand.remove(stack_hand as usize);
    }

    // Take a Move-Object and translate it to the corresponding functions
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

    // Refill the player's hand with cards to five (called before every turn and if the player plays all cards from HAND during the turn)
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

    // Check for win by checking if any player's stack is empty
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

    // Return Vec with every move that a player could make. The player then selects on of these to execute
    fn get_valid_moves(&self, playing_field: [(i8, bool); 4], hand: Vec<i8>, side: [Vec<i8>; 4], stack: Vec<i8>) -> Vec<Move> {
        let mut valid_moves = Vec::<Move>::new();

        // Check if the player can place any card from the STACK
        for (index, f) in playing_field.iter().enumerate() {
            let l = stack.last().expect("empty stack");
            if l == &(f.0 + 1) || (f.0 == 12 && l == &1) || (l == &-1 && !f.1) {
                valid_moves.push(Move { from: CardStack::Stack, from_num: 0, to: CardStack::Field, to_num: index as i8 })
            }
        }

        // Check if the player can place any card from the HAND
        for (index_f, f) in playing_field.iter().enumerate() {
            for (index_h, h) in hand.iter().enumerate() {
                if f.0 + 1 == *h || (h == &-1 && !f.1) || (f.0 == 12 && h == &1) {
                    valid_moves.push(Move { from: CardStack::Hand, from_num: index_h as i8, to: CardStack::Field, to_num: index_f as i8 })
                }
            }
        }

        // Check if the player can place any card from the SIDE
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

    // Let the player play a card, if he is done he returns false
    fn play_move(&mut self, player_num: i8, player: &Box<dyn Player>) -> bool {
        let playing_field = self.playing_field;
        let p = &self.players[player_num as usize].clone();

        // Get all valid moves the player could make
        let valid_moves = self.get_valid_moves(playing_field, p.hand.clone(), p.side.clone(), p.stack.clone());

        // If the player cannot play anything to FIELD anymore, he has to play one card to SIDE
        if valid_moves.is_empty() {
            // Let player choose which card to play from HAND to SIDE and execute it
            let move_to_side = &player.select_stack(p.hand.clone(), p.side.clone());
            self.execute_move(player_num, move_to_side);

            // Return false to end move
            false
        } else {
            // Let player select move to play
            let opponent_stack = self.players[((player_num + 1) % (self.players.len() as i8)) as usize].stack.last().unwrap();
            let selected_move = player.select_move(valid_moves, *p.stack.last().unwrap(), *opponent_stack, p.side.clone(), p.hand.clone(), playing_field);

            match selected_move {
                None => {
                    // Put card to side if player executes no move and end turn
                    self.execute_move(player_num, &player.select_stack(p.hand.clone(), p.side.clone()));
                    return false
                }
                Some(m) => {
                    // Execute the move the player chose
                    self.execute_move(player_num, &m);
                }
            }

            // Return true to signal that the player wants to perform another turn
            true
        }
    }

    // Let player play an entire turn
    fn play(&mut self, player_num: i8, player: &Box<dyn Player>) {
        // Refill HAND at the beginning
        self.refill_hand(player_num);

        let mut state = true;
        while state {
            // Check for win before letting the player play to avoid exceptions because of an empty STACK
            if self.check_win() {
                break;
            }

            // Let player play a move
            state = self.play_move(player_num, player);

            // If the player's HAND is empty and he still hasn't finished his turn, refill it
            if self.players[player_num as usize].hand.is_empty() && state {
                self.refill_hand(player_num);
            }
        }
    }
}