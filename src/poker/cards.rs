#[derive(Debug)]
pub enum Value {
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eigth,
	Nine,
	Ten,
	Jack,
	Queen,
	King,
	Ace,
}

#[derive(Debug)]
pub enum Suite {
	Clubs,
	Diamonds,
	Hearts,
	Spades,
}

#[derive(Debug)]
pub struct Card {
	pub suite: Suite,
	pub value: Value,
}
