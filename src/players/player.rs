use crate::move_stack::Move;
use rand::{Rng};

pub trait NewPlayerState {
    fn new(stack_size: i32) -> Self;
}

#[derive(Clone)]
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

pub trait Player {
    fn select_move(&self, moves: Vec<Move>, stack: i8, opponent_stack: i8, side: [Vec<i8>; 4], hand: Vec<i8>, playing_field: [(i8, bool); 4]) -> Option<Move>;
    fn select_stack(&self, hand: Vec<i8>, side: [std::vec::Vec<i8>; 4]) -> Move;
}