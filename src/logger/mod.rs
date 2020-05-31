use std::sync::{Arc, Mutex};

mod hooks;
mod simplifiers;

pub use hooks::*;
pub use simplifiers::*;

/// A logger receive each [`Log`](struct.Log.html).
///
/// Then it pass each log through its own `dispatcher`.
///
/// After its use, it will call its `dropper` when it will be dropped from
/// memory.
///
/// # See also
///
/// [`ConcurrentLogger`](type.ConcurrentLogger.html)
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

	pub fn push(&self, attributes: Vec<(String, String)>, content: Option<&str>) {
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

/// This is a wrapped logger that is safe to use in concurrency context.
///
/// # See also
///
/// - [`concurrent_logger_from`](fn.concurrent_logger_from.html)
/// - [`push`](fn.push.html)
pub type ConcurrentLogger = Arc<Mutex<Logger>>;
