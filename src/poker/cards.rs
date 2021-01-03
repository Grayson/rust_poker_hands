use super::values;

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
	pub value: values::Value,
}

impl Card {
	pub fn new(value: values::Value) -> Card {
		Card {
			suite: Suite::Spades,
			value: value,
		}
	}
}
