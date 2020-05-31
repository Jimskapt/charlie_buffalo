use std::sync::{Arc, Mutex};

/// A dispatcher is a function (or closure) called on each log received.
///
/// Inside this function, you can process the received log, like by example :
/// - modify it (by cloning it then use this clone) to add date time
/// - save it in a file
/// - displaying it in stdout and/or stderr
/// - send it to a web API through HTTP or TCP client call
///
/// It should be safe to use in concurrent context.
///
/// # Creation
///
/// You can create it with an helper :
///
/// ```rust
/// let dispatcher: charlie_buffalo::Dispatcher =
///		charlie_buffalo::new_dispatcher(Box::new(|log: charlie_buffalo::Log| {
///			println!("{}", log);
///		}));
/// ```
///
/// Or you can create it manually :
///
/// ```rust
/// let dispatcher: charlie_buffalo::Dispatcher =
///		std::sync::Arc::new(std::sync::Mutex::new(|log: charlie_buffalo::Log| {
///			println!("{}", log);
///		}));
/// ```
///
/// # Example
///
/// See an example in
/// [/examples/full_demo.rs](https://github.com/Jimskapt/charlie_buffalo/blob/master/examples/full_demo.rs)
/// in source repository.
///
/// # See also
///
/// [`new_dispatcher`](fn.new_dispatcher.html)
pub type Dispatcher = Arc<Mutex<dyn Fn(crate::Log) + Send + 'static>>;

/// A function (or closure) called when logger is freed from memory.
///
/// Inside this function, you can do some actions like by example :
/// - send a last log to save that the program is closed
///   (if logger is effectively dropped at the end of the program)
/// - display a message to user to say where are the logs
///   (in case of you save it in file(s) in [`Dispatcher`](type.Dispatcher.html))
///
/// It is optional to use in logger, because maybe you don't need to do
/// something when logger is freed.
///
/// It should be safe to use in concurrent context.
///
/// # Creation
///
/// You can create it with an helper :
///
/// ```rust
/// let dropper: charlie_buffalo::Dropper =
///		charlie_buffalo::new_dropper(Box::new(|_logger: &charlie_buffalo::Logger| {
///			println!("The logger is now freed from memory");
///		}));
/// ```
///
/// Or you can create it manually :
///
/// ```rust
/// let dropper: charlie_buffalo::Dropper =
///		Some(std::sync::Arc::new(std::sync::Mutex::new(|_logger: &charlie_buffalo::Logger| {
///			println!("The logger is now freed from memory");
///		})));
/// ```
///
/// # Example
///
/// See an example in
/// [/examples/full_demo.rs](https://github.com/Jimskapt/charlie_buffalo/blob/master/examples/full_demo.rs)
/// in source repository.
///
/// # See also
///
/// - [`new_dropper`](fn.new_dropper.html)
/// - [`std::ops::Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html)
pub type Dropper = Option<Arc<Mutex<dyn Fn(&super::Logger) + Send + 'static>>>;
