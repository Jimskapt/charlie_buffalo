use charlie_buffalo as cb;

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

impl std::convert::Into<cb::AttributeAsString> for Level {
	fn into(self) -> cb::AttributeAsString {
		return (String::from("level"), cb::ValueAsString::as_string(&self));
	}
}

const LOG_FILE_PATH: &str = "logs.msgpack";

fn dispatcher(log: cb::Log) {
	let mut new_log = log.clone();

	let attributes: Vec<cb::AttributeAsString> =
		vec![cb::Attr::from("time", format!("{}", chrono::offset::Local::now())).into()];
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

fn main() {
	std::fs::write(LOG_FILE_PATH, "").unwrap();

	let logger = cb::Logger::new(
		Box::new(dispatcher),
		Some(Box::new(|logger: &cb::Logger| {
			logger.push(vec![cb::Flag::from("STOP").into()], None);

			println!(
				"(please also read file {} to see written logs)\n",
				LOG_FILE_PATH
			);
		})),
	);

	/* TODO
	std::panic::set_hook(Box::new(|infos| {
		match infos.location() {
			Some(location) => {
				logger.push(vec![
					cb::Attr::from("level", Level::PANIC).into(),
					cb::Attr::from("code", format!("{}:{}", location.file(), location.line())).into(),
				],
				"panic");
			},
			None => {
				logger.push(vec![
					cb::Attr::from("level", Level::PANIC).into(),
				],
				"panic");
			}
		}
	}));
	*/

	logger.push(vec![cb::Flag::from("STARTUP").into()], None);
	logger.push(
		vec![
			Level::DEBUG.into(),
			cb::Attr::from("code", format!("{}:{}", file!(), line!())).into(),
			cb::Attr::from("functionality", "I'm \"quoting\" for tests ...").into(),
		],
		Some("logger created"),
	);
	logger.push(
		vec![
			Level::INFO.into(),
			cb::Attr::from("user_id", &(48625 as usize)).into(),
			cb::Attr::from("code", format!("{}:{}", file!(), line!())).into(),
		],
		Some("user has log-in"),
	);
	logger.push(
		vec![
			Level::WARN.into(),
			cb::Attr::from("logged", true).into(),
			cb::Attr::from("code", format!("{}:{}", file!(), line!())).into(),
			cb::Attr::from("credential_level", 'D').into(),
		],
		Some("token cookie is not readable"),
	);
	logger.push(
		vec![
			Level::ERROR.into(),
			cb::Attr::from("HTTP-code", &(404 as u16)).into(),
			cb::Attr::from("route", "/users/16472/friends").into(),
			cb::Attr::from("code", format!("{}:{}", file!(), line!())).into(),
			cb::Attr::from(
				"IPs",
				vec!["127.0.0.1", "localhost", "::1", "0:0:0:0:0:0:0:1"],
			)
			.into(),
		],
		Some("this is first ERROR"),
	);

	let result: Vec<cb::Log> =
		rmp_serde::decode::from_read(std::fs::File::open(LOG_FILE_PATH).unwrap())
			.unwrap_or_default();
	std::fs::write(
		"logs.converted.json",
		serde_json::ser::to_vec_pretty(&result).unwrap(),
	)
	.unwrap();

	println!();

	/* TODO
	panic!("Here we voluntary finishing this example with this panic, which will be also logged !");
	*/
}
