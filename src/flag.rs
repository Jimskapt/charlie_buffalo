/// A `Flag` is an [`Attribute`](struct.Attribute.html) with a key, but with an
/// empty `value`.
pub struct Flag {
	value: String,
}

impl<'a> std::convert::Into<(String, String)> for Flag {
	fn into(self) -> (String, String) {
		return (self.value, String::new());
	}
}

impl std::convert::From<&str> for Flag {
	fn from(key: &str) -> Self {
		Flag {
			value: String::from(key),
		}
	}
}
