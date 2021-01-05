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

pub fn determine_high_hand(cards: &[Card]) -> Hand {
	get_hands_made_of_duplicates(cards)
		.or(Some(Hand::HighCard))
		.unwrap()
}

fn get_hands_made_of_duplicates(cards: &[Card]) -> Option<Hand> {
	let counts = card_counts(cards);
	match counts.values().max().unwrap() {
		5 => Some(Hand::FiveOfAKind),
		4 => Some(Hand::FourOfAKind),
		3 => Some(Hand::ThreeOfAKind),
		2 => Some(distinguish_pairs_from_two_pairs(&counts)),
		_ => None,
	}
}

fn distinguish_pairs_from_two_pairs(counts: &HashMap<Value, i32>) -> Hand {
	let number_of_pairs = counts.iter().fold(0, |acc, x| acc + ((*x.1 == 2) as i32));
	match number_of_pairs {
		2 => Hand::TwoPair,
		1 => Hand::OnePair,
		_ => panic!("Invalid hand passed in to distinguish two pair from a pair.")
	}
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

	#[test]
	fn test_high_card() {
		let cards = [
			Card::new(Suite::Clubs, Value::Ace),
			Card::new(Suite::Clubs, Value::Two),
			Card::new(Suite::Clubs, Value::Three),
			Card::new(Suite::Clubs, Value::Four),
			Card::new(Suite::Clubs, Value::Five),
		];

		assert_eq!(Hand::HighCard, determine_high_hand(&cards));
	}

	#[test]
	fn test_pair() {
		let cards = [
			Card::new(Suite::Clubs, Value::Ace),
			Card::new(Suite::Clubs, Value::Ace),
			Card::new(Suite::Clubs, Value::Three),
			Card::new(Suite::Clubs, Value::Four),
			Card::new(Suite::Clubs, Value::Five),
		];

		assert_eq!(Hand::OnePair, determine_high_hand(&cards));
	}

	#[test]
	fn test_two_pair() {
		let cards = [
			Card::new(Suite::Clubs, Value::Ace),
			Card::new(Suite::Diamonds, Value::Ace),
			Card::new(Suite::Clubs, Value::Three),
			Card::new(Suite::Clubs, Value::Three),
			Card::new(Suite::Clubs, Value::Five),
		];

		assert_eq!(Hand::TwoPair, determine_high_hand(&cards));
	}

	#[test]
	fn test_three_of_a_kind() {
		let cards = [
			Card::new(Suite::Clubs, Value::Ace),
			Card::new(Suite::Diamonds, Value::Ace),
			Card::new(Suite::Hearts, Value::Ace),
			Card::new(Suite::Clubs, Value::Four),
			Card::new(Suite::Clubs, Value::Five),
		];

		assert_eq!(Hand::ThreeOfAKind, determine_high_hand(&cards));
	}

	#[test]
	fn test_four_of_a_kind() {
		let cards = [
			Card::new(Suite::Clubs, Value::Ace),
			Card::new(Suite::Diamonds, Value::Ace),
			Card::new(Suite::Hearts, Value::Ace),
			Card::new(Suite::Spades, Value::Ace),
			Card::new(Suite::Clubs, Value::Five),
		];

		assert_eq!(Hand::FourOfAKind, determine_high_hand(&cards));
	}

	#[test]
	fn test_five_of_a_kind() {
		let cards = [
			Card::new(Suite::Clubs, Value::Ace),
			Card::new(Suite::Diamonds, Value::Ace),
			Card::new(Suite::Hearts, Value::Ace),
			Card::new(Suite::Spades, Value::Ace),
			Card::new(Suite::Clubs, Value::Ace), // No wildcards implemented yet
		];

		assert_eq!(Hand::FiveOfAKind, determine_high_hand(&cards));
	}
}
