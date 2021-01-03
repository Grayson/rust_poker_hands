#[derive(Debug, PartialEq, PartialOrd)]
pub enum Value {
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Jack,
	Queen,
	King,
	Ace,
}

pub fn from_string(slice: &str) -> Option<Value> {
	let is_ten = || slice.len() == 2 && &slice[0..2] == "10";

	if slice.is_empty() { None }
	else if is_ten() { Some(Value::Ten) }
	else { from_char(slice.chars().next().unwrap()) }
}

fn from_char(ch: char) -> Option<Value> {
	match ch {
		'2' => Some(Value::Two),
		'3' => Some(Value::Three),
		'4' => Some(Value::Four),
		'5' => Some(Value::Five),
		'6' => Some(Value::Six),
		'7' => Some(Value::Seven),
		'8' => Some(Value::Eight),
		'9' => Some(Value::Nine),
		// '10' => Unable to be represented by a single char
		'j' | 'J' => Some(Value::Jack),
		'q' | 'Q' => Some(Value::Queen),
		'k' | 'K' => Some(Value::King),
		'a' | 'A' => Some(Value::Ace),
		_ => None,
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_chars() {
		let tests = vec![
			('2', Value::Two),
			('3', Value::Three),
			('4', Value::Four),
			('5', Value::Five),
			('6', Value::Six),
			('7', Value::Seven),
			('8', Value::Eight),
			('9', Value::Nine),
			// ('10', Value::Ten),
			('J', Value::Jack),
			('Q', Value::Queen),
			('K', Value::King),
			('A', Value::Ace),
		];

		for (ch, expectation) in tests {
			let value = from_char(ch);
			assert_eq!(expectation, value.unwrap());	
		}
	}

	#[test]
	fn test_invalid() {
		assert_eq!(from_char('B'), None);
	}

	#[test]
	fn test_strings() {
		let tests = vec![
			("2", Value::Two),
			("3", Value::Three),
			("4", Value::Four),
			("5", Value::Five),
			("6", Value::Six),
			("7", Value::Seven),
			("8", Value::Eight),
			("9", Value::Nine),
			("10", Value::Ten),
			("J", Value::Jack),
			("Q", Value::Queen),
			("K", Value::King),
			("A", Value::Ace),

			("200", Value::Two),
		];

		for (test, expectation) in tests {
			let value = from_string(test);
			let result = match value {
				Some(x) => x,
				None => panic!("Unable to convert {} to {:?}", test, expectation),
			};
			assert_eq!(expectation, result);
		}
	}
}
