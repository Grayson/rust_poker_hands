use super::values;
use super::suites;

#[derive(Debug)]
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
