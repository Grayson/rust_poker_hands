use poker_hands::poker::{cards, values};

fn main() {
    println!("{:?}", cards::Card::new(values::Value::Ace));
    println!("{:?}", values::Value::Seven);
}
