/// An unique log.
///
/// It has `attributes` which can be easily constructed by
/// [`Vec<Attribute>`](struct.Attribute.html).
///
/// Its `content` is optional, in case of all important data is already in
/// `attributes`.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Log {
	pub attributes: std::collections::BTreeMap<String, String>,
	pub content: Option<String>,
}

impl std::convert::From<(Vec<(String, String)>, Option<&str>)> for Log {
	fn from(input: (Vec<(String, String)>, Option<&str>)) -> Self {
		let (attributes, content) = input;

		let mut temp = std::collections::BTreeMap::new();
		for attribute in attributes {
			temp.insert(attribute.0, attribute.1);
		}

		Log {
			attributes: temp,
			content: match content {
				Some(content) => Some(String::from(content)),
				None => None,
			},
		}
	}
}

impl std::fmt::Display for Log {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "-> ")?;
		if let Some(content) = &self.content {
			writeln!(f, "{}", &content)?;
		} else {
			writeln!(f)?;
		}

		for (key, value) in &self.attributes {
			if value != "" {
				writeln!(f, "\t\"{}\" : {}", &key, &value)?;
			} else {
				writeln!(f, "\t> {}", &key)?;
			}
		}

		Ok(())
	}
}
