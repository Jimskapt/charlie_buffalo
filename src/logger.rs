type Dispatcher = Box<dyn Fn(crate::Log)>;
type Hook = Option<Box<dyn Fn(crate::Log) -> Option<crate::Log>>>;

pub struct Logger {
	hook: Hook,
	dispatcher: Dispatcher,
}

impl Logger {
	pub fn new(hook: Hook, dispatcher: Dispatcher) -> Self {
		Logger { hook, dispatcher }
	}

	pub fn push(&self, attributes: Vec<crate::AttributeAsString>, content: &str) {
		let mut temp = std::collections::BTreeMap::new();
		for attribute in attributes {
			temp.insert(attribute.0, attribute.1);
		}

		let log = Some(crate::Log {
			attributes: temp,
			content: String::from(content),
		});

		let automated_log: Option<crate::Log> = if let Some(hook) = &self.hook {
			(hook)(log.unwrap())
		} else {
			log
		};

		if let Some(log) = automated_log {
			(self.dispatcher)(log);
		}
	}
}
