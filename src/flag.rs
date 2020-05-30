pub struct Flag {}
impl Flag {
	pub fn from(key: &str) -> crate::Attribute<&str> {
		return crate::Attribute::from(key, "");
	}
}
