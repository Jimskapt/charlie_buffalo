use std::sync::{Arc, Mutex};

pub type Dispatcher = Arc<Mutex<dyn Fn(crate::Log) + Send + 'static>>;
pub type Dropper = Option<Arc<Mutex<dyn Fn(&Logger) + Send + 'static>>>;

pub type AsyncLogger = Arc<Mutex<Logger>>;

pub struct Logger {
	dispatcher: Dispatcher,
	dropper: Dropper,
}

impl Logger {
	pub fn new(dispatcher: Dispatcher, dropper: Dropper) -> AsyncLogger {
		let result = Logger {
			dispatcher,
			dropper,
		};

		let logger: Arc<Mutex<Logger>> = Arc::from(Mutex::from(result));

		return logger;
	}

	pub fn push(&self, attributes: Vec<crate::AttributeAsString>, content: Option<&str>) {
		let mut temp = std::collections::BTreeMap::new();
		for attribute in attributes {
			temp.insert(attribute.0, attribute.1);
		}

		(self.dispatcher.lock().unwrap())(crate::Log {
			attributes: temp,
			content: match content {
				Some(content) => Some(String::from(content)),
				None => None,
			},
		});
	}

	pub fn receive(&self, log: crate::Log) {
		(self.dispatcher.lock().unwrap())(log);
	}
}

impl Drop for Logger {
	fn drop(&mut self) {
		if let Some(dropper) = &self.dropper {
			(dropper.lock().unwrap())(self);
		}
	}
}
