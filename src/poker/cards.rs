use super::values;
use super::suites;
use std::cmp;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Card {
	pub suite: suites::Suite,
	pub value: values::Value,
}

impl Card {
	pub fn new(suite: suites::Suite, value: values::Value) -> Card {
		Card {
			suite: suite,
			value: value,
		}
	}
}

impl PartialOrd for Card {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		self.value.partial_cmp(&other.value)
	}
}

fn from_ten_string(string: &str) -> Option<Card> {
	if string.len() != 3 { return None }

	let ten = match values::from_string(&string[0..2]) {
		Some(x) => Some(x),
		None => values::from_string(&string[1..3]),
	};
	if ten.is_none() { return None }

	let mut chars = string.chars();
	let suite = match suites::from_char(chars.next().unwrap()) {
		Some(x) => Some(x),
		None => suites::from_char(chars.skip(1).next().unwrap()),
	};
	
	if ten.is_some() && suite.is_some() { Some(Card::new(suite.unwrap(), ten.unwrap())) }
	else { None }
}

fn from_two_char_card(string: &str) -> Option<Card> {
	if string.len() != 2 { return None }

	let value = match values::from_string(&string[0..1]) {
		Some(x) => Some(x),
		None => values::from_string(&string[1..2]),
	};
	if value.is_none() { return None }

	let mut chars = string.chars();
	let suite = match suites::from_char(chars.next().unwrap_or_default()) {
		Some(x) => Some(x),
		None => suites::from_char(chars.next().unwrap_or_default()),
	};

	if value.is_some() && suite.is_some() { 
		Some(Card::new(suite.unwrap(), value.unwrap())) 
	}
	else { None }
}

fn reify_slice(string: &str) -> Option<Card> {
	match from_two_char_card(string) {
		Some(x) => Some(x),
		None => from_ten_string(string),
	}
}

pub fn collect_cards(data: &str) -> Vec<Card> {
	data.split(|c: char| !c.is_alphanumeric())
		.filter(|x| !x.is_empty())
		.map(|x| reify_slice(x))
		.filter(|x| x.is_some())
		.map(|x| x.unwrap())
		.collect()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn collect_ten_string() {
		let ten_of_hearts = Card {
			suite: suites::Suite::Hearts,
			value: values::Value::Ten,
		};
		assert_eq!(ten_of_hearts, from_ten_string("10H").unwrap());
		assert_eq!(ten_of_hearts, from_ten_string("H10").unwrap());
	}

	#[test]
	fn collect_two_card_string() {
		let ace_of_spades = Card {
			suite: suites::Suite::Spades,
			value: values::Value::Ace,
		};

		assert_eq!(ace_of_spades, from_two_char_card("AS").unwrap());
		assert_eq!(ace_of_spades, from_two_char_card("aS").unwrap());
		assert_eq!(ace_of_spades, from_two_char_card("as").unwrap());
		assert_eq!(ace_of_spades, from_two_char_card("SA").unwrap());
		assert_eq!(ace_of_spades, from_two_char_card("Sa").unwrap());
		assert_eq!(ace_of_spades, from_two_char_card("sa").unwrap());
	}

	#[test]
	fn collect_card() {
		let test = "AC";
		let cards = collect_cards(test);
		assert_eq!(1, cards.len());
	}

	#[test]
	fn test_collect_cards() {
		let test = "AC;2D 3H,*$5X1S+4S";
		let cards = collect_cards(test);
		assert_eq!(4, cards.len());
	}
}
