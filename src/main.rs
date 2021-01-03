use poker_hands::poker;

fn main() {
    println!("{:?}", poker::cards::Card {
        value: poker::cards::Value::Ace,
        suite: poker::cards::Suite::Spades,
    });
}
