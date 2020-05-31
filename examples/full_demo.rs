// following line is optional, it is useful only to fetch crate
// in https://play.rust-lang.org/ when copying/pasting this code.
extern crate charlie_buffalo;

use charlie_buffalo as cb;

const LOG_FILE_PATH: &str = "logs.msgpack";

fn main() {
	std::fs::write(LOG_FILE_PATH, "").unwrap();

	let logger = cb::concurrent_logger_from(cb::Logger::new(
		cb::new_dispatcher(Box::from(dispatcher)),
		cb::new_dropper(Box::from(dropper))
	));

	let logger_for_panic = logger.clone();
	std::panic::set_hook(Box::new(move |infos| {
		let attributes = match infos.location() {
			Some(location) => vec![
				Level::PANIC.into(),
				cb::Flag::from("STOP").into(),
				cb::Attr::new("code", format!("{}:{}", location.file(), location.line())).into(),
			],
			None => vec![Level::PANIC.into()],
		};

		cb::push(
			&logger_for_panic,
			attributes,
			Some(&format!(
				"{:?}",
				infos.payload().downcast_ref::<&str>().unwrap_or(&"")
			)),
		);

		// following is duplicate
		let result: Vec<cb::Log> =
			rmp_serde::decode::from_read(std::fs::File::open(LOG_FILE_PATH).unwrap())
				.unwrap_or_default();
		std::fs::write(
			"logs.converted.json",
			serde_json::ser::to_vec_pretty(&result).unwrap(),
		)
		.unwrap();

		println!(
			"(please also read file {} and logs.converted.json to see written logs)\n",
			LOG_FILE_PATH
		);
	}));

	// manual concurrent push :
	logger
		.lock()
		.unwrap()
		.push(vec![cb::Flag::from("STARTUP").into()], None);

	// easier current push :
	cb::push(
		&logger,
		vec![
			Level::DEBUG.into(),
			cb::Attr::new("code", format!("{}:{}", file!(), line!())).into(),
			cb::Attr::new("functionality", "I'm \"quoting\" for tests ...").into(),
		],
		Some("logger created"),
	);
	cb::push(
		&logger,
		vec![
			Level::INFO.into(),
			cb::Attr::new("user_id", &(48625 as usize)).into(),
			cb::Attr::new("code", format!("{}:{}", file!(), line!())).into(),
		],
		Some("user has log-in"),
	);
	cb::push(
		&logger,
		vec![
			Level::WARN.into(),
			cb::Attr::new("logged", true).into(),
			cb::Attr::new("code", format!("{}:{}", file!(), line!())).into(),
			cb::Attr::new("credential_level", 'D').into(),
		],
		Some("token cookie is not readable"),
	);
	cb::push(
		&logger,
		vec![
			Level::ERROR.into(),
			cb::Attr::new("HTTP-code", &(404 as u16)).into(),
			cb::Attr::new("route", "/users/16472/friends").into(),
			cb::Attr::new("code", format!("{}:{}", file!(), line!())).into(),
			cb::Attr::new(
				"IPs",
				vec!["127.0.0.1", "localhost", "::1", "0:0:0:0:0:0:0:1"],
			)
			.into(),
		],
		Some("this is first ERROR"),
	);

	panic!("Here we voluntary finishing this example with this panic, which will be also logged !");

	/*
	following line needed to run Drop on Arc<Mutex<Logger>
	(otherwise the hard counter stays to 1 at the end of main do drop() is not run)
	*/

	let _ = std::panic::take_hook();
}

fn dispatcher(log: cb::Log) {
	let mut new_log = log.clone();

	let attributes: Vec<(String, String)> =
		vec![cb::Attr::new("time", format!("{}", chrono::offset::Local::now())).into()];
	for attribute in attributes {
		new_log.attributes.insert(attribute.0, attribute.1);
	}

	match new_log.attributes.get("level") {
		Some(level) => {
			if level == Level::PANIC || level == Level::ERROR {
				eprintln!("{}", &new_log);
			} else {
				println!("{}", &new_log);
			}
		}
		_ => {
			println!("{}", &new_log);
		}
	}

	let mut result: Vec<cb::Log> =
		rmp_serde::decode::from_slice(std::fs::read(LOG_FILE_PATH).unwrap_or_default().as_slice())
			.unwrap_or_default();
	result.push(new_log);
	std::fs::write(LOG_FILE_PATH, rmp_serde::encode::to_vec(&result).unwrap()).ok();
}

fn dropper(logger: &cb::Logger) {
	logger.push(vec![cb::Flag::from("STOP").into()], None);

	let result: Vec<cb::Log> =
		rmp_serde::decode::from_read(std::fs::File::open(LOG_FILE_PATH).unwrap())
			.unwrap_or_default();
	std::fs::write(
		"logs.converted.json",
		serde_json::ser::to_vec_pretty(&result).unwrap(),
	)
	.unwrap();

	println!(
		"(please also read file {} and logs.converted.json to see written logs)\n",
		LOG_FILE_PATH
	);
}

#[derive(serde::Serialize)]
enum Level {
	DEBUG,
	INFO,
	WARN,
	ERROR,
	PANIC,
}

impl cb::ValueAsString for Level {
	fn as_string(&self) -> String {
		format!(
			"{}",
			match self {
				Level::DEBUG => 10,
				Level::INFO => 20,
				Level::WARN => 30,
				Level::ERROR => 40,
				Level::PANIC => 50,
			}
		)
	}
}

impl std::cmp::PartialEq<Level> for &String {
	fn eq(&self, other: &Level) -> bool {
		*self == &charlie_buffalo::ValueAsString::as_string(other)
	}
}

impl std::convert::Into<(String, String)> for Level {
	fn into(self) -> (String, String) {
		return (String::from("level"), cb::ValueAsString::as_string(&self));
	}
}
