#[derive(Debug, PartialEq)]
pub enum Suite {
	Clubs,
	Diamonds,
	Hearts,
	Spades,
}

impl Suite {
	pub fn from_char(ch: char) -> Option<Suite> {
		match ch {
			'c' | 'C' => Some(Suite::Clubs),
			'd' | 'D' => Some(Suite::Diamonds),
			'h' | 'H' => Some(Suite::Hearts),
			's' | 'S' => Some(Suite::Spades),
			_ => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_from_char() {
		let tests = vec![
			('c', Suite::Clubs),
			('C', Suite::Clubs),
			('d', Suite::Diamonds),
			('D', Suite::Diamonds),
			('h', Suite::Hearts),
			('H', Suite::Hearts),
			('s', Suite::Spades),
			('S', Suite::Spades),
		];

		for (test, expectation) in tests {
			let value = Suite::from_char(test);
			let result = match value {
				Some(x) => x,
				None => panic!("Unable to convert {} to {:?}", test, expectation),
			};
			assert_eq!(expectation, result);
		}
	}

	#[test]
	fn test_from_invalid_char() {
		assert_eq!(None, Suite::from_char('X'));
	}
}
