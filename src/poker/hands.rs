use std::collections::HashMap;

use super::cards::Card;
use super::values::Value;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Hand {
	HighCard,
	OnePair,
	TwoPair,
	ThreeOfAKind,
	Straight,
	Flush,
	FullHouse,
	FourOfAKind,
	StraightFlush,
	FiveOfAKind,
}

pub fn card_counts(cards: &[Card]) -> HashMap<Value, i32>
{
	let mut map = HashMap::new();
	for card in cards {
		let counter = map.entry(card.value).or_insert(0);
		*counter += 1;
	}
	return map;
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::super::suites::Suite;

	#[test]
	fn test_card_counts() {
		let cards = [
			Card::new(Suite::Clubs, Value::Three),
			Card::new(Suite::Diamonds, Value::Three),
			Card::new(Suite::Clubs, Value::Two),
			Card::new(Suite::Clubs, Value::Three),
			Card::new(Suite::Clubs, Value::Two),
			Card::new(Suite::Clubs, Value::Ace),
		];

		let counts = card_counts(&cards);

		assert_eq!(3, counts[&Value::Three]);
		assert_eq!(2, counts[&Value::Two]);
		assert_eq!(1, counts[&Value::Ace]);
	}
}
