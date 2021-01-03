use super::cards::Card;
use super::suites::Suite;
use super::values::Value;

#[derive(Debug)]
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

fn get_longest_card_count<'a>(cards: &'a [Card]) -> (&'a Card, i32) {
	if cards.len() == 0 {
		panic!("Cannot pass in empty cards")
	}

	let mut max_card = &cards[0];
	let mut max_count = 0;

	for card in cards {
		let count = cards.iter().fold(0, |acc, x| acc + ((x.value == card.value) as i32) );
		if count >= max_count {
			max_count = count;
			max_card = if *card > *max_card { card } else { max_card }
		}
	}

	(max_card, max_count)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_get_longest_card_count() {
		let tests = vec![
			(Card::new(Suite::Clubs, Value::Ace), 1, [
				Card::new(Suite::Clubs, Value::Ace),
				Card::new(Suite::Clubs, Value::Two),
				Card::new(Suite::Clubs, Value::Three),
				Card::new(Suite::Clubs, Value::Four),
				Card::new(Suite::Clubs, Value::Five),
			]),
			(Card::new(Suite::Clubs, Value::Ace), 2, [
				Card::new(Suite::Clubs, Value::Ace),
				Card::new(Suite::Diamonds, Value::Ace),
				Card::new(Suite::Clubs, Value::Three),
				Card::new(Suite::Clubs, Value::Four),
				Card::new(Suite::Clubs, Value::Five),
			]),
			(Card::new(Suite::Clubs, Value::Ace), 3, [
				Card::new(Suite::Clubs, Value::Ace),
				Card::new(Suite::Diamonds, Value::Ace),
				Card::new(Suite::Hearts, Value::Ace),
				Card::new(Suite::Clubs, Value::Four),
				Card::new(Suite::Clubs, Value::Five),
			]),
		];

		for test in tests {
			let (expected_card, expected_count, cards) = test;
			let (card, count) = get_longest_card_count(&cards);
			assert_eq!(expected_card, *card);
			assert_eq!(expected_count, count);
		}
	}

	#[test]
	#[should_panic]
	fn test_panic_on_empty_cards() {
		get_longest_card_count(&[]);
	}
}
