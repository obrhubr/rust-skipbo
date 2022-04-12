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