use std::sync::{Arc, Mutex};

/// A simplifier to create a [`Dispatcher`](type.Dispatcher.html) without
/// concurrent boilerplate.
pub fn new_dispatcher(function: Box<dyn Fn(crate::Log) + Send>) -> super::Dispatcher {
	return Arc::new(Mutex::new(function));
}

/// A simplifier to create a [`Dropper`](type.Dropper.html) without
/// concurrent boilerplate.
pub fn new_dropper(function: Box<dyn Fn(&super::Logger) + Send>) -> super::Dropper {
	return Some(Arc::new(Mutex::new(function)));
}

/// A simplifier to create a [`ConcurrentLogger`](type.ConcurrentLogger.html)
/// without concurrent boilerplate.
pub fn concurrent_logger_from(logger: super::Logger) -> super::ConcurrentLogger {
	return Arc::new(Mutex::new(logger));
}

/// A simplifier to send a log in a
/// [`ConcurrentLogger`](type.ConcurrentLogger.html) without concurrent
/// boilerplate.
pub fn push(
	logger: &super::ConcurrentLogger,
	attributes: Vec<(String, String)>,
	content: Option<&str>,
) {
	match logger.lock() {
		Ok(logger) => {
			logger.push(attributes, content);
		}
		Err(_) => {
			eprintln!(
				"ERROR: unable to push following log :\n{}",
				crate::Log::from((attributes, content))
			);
		}
	}
}
