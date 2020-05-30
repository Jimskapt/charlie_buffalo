#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Log {
	pub attributes: std::collections::BTreeMap<String, String>,
	pub content: Option<String>,
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
