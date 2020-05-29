pub type AttributeAsString = (String, String);

pub type Attr<T> = Attribute<T>;

pub struct Attribute<T: crate::ValueAsString> {
	key: String,
	value: T,
}

impl<T: crate::ValueAsString> Attribute<T> {
	pub fn from(key: &str, value: T) -> Self {
		return Attribute {
			key: String::from(key),
			value,
		};
	}
}

impl<T: crate::ValueAsString> std::convert::Into<AttributeAsString> for Attribute<T> {
	fn into(self) -> AttributeAsString {
		return (self.key.clone(), self.value.as_string());
	}
}
