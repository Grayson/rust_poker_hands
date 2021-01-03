use poker_hands::poker::{cards, values, suites};

fn main() {
    println!("{:?}", cards::Card::new(suites::Suite::Spades, values::Value::Ace));
    println!("{:?}", values::Value::Seven);
}
