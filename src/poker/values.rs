#[derive(Debug, PartialEq, PartialOrd)]
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

impl Value {
	pub fn from_char(ch: char) -> Option<Value> {
		match ch {
			'A' => Some(Value::Ace),
			_ => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_chars() {
		let value = Value::from_char('A');
		assert_eq!(Value::Ace, value.unwrap());
	}
}
