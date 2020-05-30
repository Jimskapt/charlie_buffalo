type Dispatcher = Box<dyn Fn(crate::Log)>;

pub struct Logger {
	dispatcher: Dispatcher,
}

impl Logger {
	pub fn new(dispatcher: Dispatcher) -> Self {
		Logger { dispatcher }
	}

	pub fn push(&self, attributes: Vec<crate::AttributeAsString>, content: &str) {
		let mut temp = std::collections::BTreeMap::new();
		for attribute in attributes {
			temp.insert(attribute.0, attribute.1);
		}

		(self.dispatcher)(crate::Log {
			attributes: temp,
			content: String::from(content),
		});
	}
}
