type Dispatcher = Box<dyn Fn(crate::Log)>;
type Dropper = Option<Box<dyn Fn(&Logger)>>;

pub struct Logger {
	dispatcher: Dispatcher,
	dropper: Dropper,
}

impl Logger {
	pub fn new(dispatcher: Dispatcher, dropper: Dropper) -> Self {
		Logger {
			dispatcher,
			dropper,
		}
	}

	pub fn push(&self, attributes: Vec<crate::AttributeAsString>, content: Option<&str>) {
		let mut temp = std::collections::BTreeMap::new();
		for attribute in attributes {
			temp.insert(attribute.0, attribute.1);
		}

		(self.dispatcher)(crate::Log {
			attributes: temp,
			content: match content {
				Some(content) => Some(String::from(content)),
				None => None,
			},
		});
	}
}

impl Drop for Logger {
	fn drop(&mut self) {
		if let Some(dropper) = &self.dropper {
			(dropper)(self);
		}
	}
}
