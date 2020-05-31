#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Log {
	pub attributes: std::collections::BTreeMap<String, String>,
	pub content: Option<String>,
}

impl std::convert::From<(Vec<crate::AttributeAsString>, Option<&str>)> for Log {
	fn from(input: (Vec<crate::AttributeAsString>, Option<&str>)) -> Self {
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
		write!(f, "-> ").unwrap();
		if let Some(content) = &self.content {
			writeln!(f, "{}", &content).unwrap();
		} else {
			writeln!(f).unwrap();
		}

		for (key, value) in &self.attributes {
			writeln!(f, "\t{} : {}", &key, &value).unwrap();
		}

		Ok(())
	}
}
