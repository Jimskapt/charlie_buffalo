use std::sync::{Arc, Mutex};

pub type Dispatcher = Arc<Mutex<dyn Fn(crate::Log) + Send + 'static>>;
pub type Dropper = Option<Arc<Mutex<dyn Fn(&Logger) + Send + 'static>>>;

pub fn new_dispatcher(function: Box<dyn Fn(crate::Log) + Send>) -> Dispatcher {
	return Arc::new(Mutex::new(function));
}

pub fn new_dropper(function: Box<dyn Fn(&Logger) + Send>) -> Dropper {
	return Some(Arc::new(Mutex::new(function)));
}

pub type ConcurrentLogger = Arc<Mutex<Logger>>;

pub fn concurrent_logger_from(logger: Logger) -> ConcurrentLogger {
	return Arc::new(Mutex::new(logger));
}

pub struct Logger {
	dispatcher: Dispatcher,
	dropper: Dropper,
}

impl Logger {
	pub fn new(dispatcher: Dispatcher, dropper: Dropper) -> Logger {
		let result = Logger {
			dispatcher,
			dropper,
		};

		return result;
	}

	pub fn push(&self, attributes: Vec<crate::AttributeAsString>, content: Option<&str>) {
		(self.dispatcher.lock().unwrap())(crate::Log::from((attributes, content)));
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

pub fn push(logger: &ConcurrentLogger, attributes: Vec<crate::AttributeAsString>, content: Option<&str>) {
	match logger.lock() {
		Ok(logger) => {
			logger.push(attributes, content);
		},
		Err(_) => {
			eprintln!("ERROR: unable to push following log :\n{}", crate::Log::from((attributes, content)));
		}
	}
}
