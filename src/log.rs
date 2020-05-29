#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Log {
	pub attributes: std::collections::BTreeMap<String, String>,
	pub content: String,
}

impl std::fmt::Display for Log {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "{}", &self.content).unwrap();

		for (key, value) in &self.attributes {
			writeln!(f, "\t{} : {}", &key, &value).unwrap();
		}

		Ok(())
	}
}
