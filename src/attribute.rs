/// An attribute is a key-value data, where value is generic.
///
/// It will allow external system to filter [`Log`](struct.Log.html) list
/// thanks to this attributes. Due to this, the value will be casted to String
/// later, thanks to [`as_string(&self)`](trait.ValueAsString.html).
///
/// # See also
///
/// [`ValueAsString`](trait.ValueAsString.html)
pub struct Attribute<T: super::ValueAsString> {
	key: String,
	value: T,
}

impl<T: super::ValueAsString> Attribute<T> {
	pub fn new(key: &str, value: T) -> Self {
		return Attribute {
			key: String::from(key),
			value,
		};
	}
}

impl<T: crate::ValueAsString> std::convert::Into<(String, String)> for Attribute<T> {
	fn into(self) -> (String, String) {
		return (self.key.clone(), self.value.as_string());
	}
}

/// A short-hand for [`Attribute`](struct.Attribute.html)
pub type Attr<T> = Attribute<T>;
