use charlie_buffalo as cb;
use charlie_buffalo::ValueAsString; // used in impl std::cmp::PartialEq<Level> for &String

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
		*self == &other.as_string()
	}
}

const LOG_FILE_PATH: &str = "logs.msgpack";

fn hook(log: cb::Log) -> Option<cb::Log> {
	let mut result = log.clone();

	let attributes: Vec<cb::AttributeAsString> =
		vec![cb::Attr::from("time", format!("{}", chrono::offset::Local::now())).into()];
	for attribute in attributes {
		result.attributes.insert(attribute.0, attribute.1);
	}

	return Some(result);
}

fn dispatcher(log: cb::Log) {
	match log.attributes.get("level") {
		Some(level) => {
			if level == Level::PANIC || level == Level::ERROR {
				eprintln!("{}", &log);
			} else {
				println!("{}", &log);
			}
		}
		_ => {
			println!("{}", &log);
		}
	}

	let mut result: Vec<cb::Log> = if std::path::PathBuf::from(LOG_FILE_PATH).exists() {
		rmp_serde::decode::from_read(std::fs::File::open(LOG_FILE_PATH).unwrap())
			.unwrap_or_default()
	} else {
		vec![]
	};
	result.push(log);
	std::fs::write(LOG_FILE_PATH, rmp_serde::encode::to_vec(&result).unwrap()).unwrap();
}

fn main() {
	std::fs::write(LOG_FILE_PATH, "").unwrap();

	let logger = cb::Logger::new(Some(Box::new(hook)), Box::new(dispatcher));

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

	logger.push(
		vec![
			cb::Attr::from("level", Level::DEBUG).into(),
			cb::Attr::from("code", format!("{}:{}", file!(), line!())).into(),
			cb::Attr::from("functionality", "I'm \"quoting\" for tests ...").into(),
		],
		"logger created",
	);
	logger.push(
		vec![
			cb::Attr::from("level", Level::INFO).into(),
			cb::Attr::from("user_id", &(48625 as usize)).into(),
			cb::Attr::from("code", format!("{}:{}", file!(), line!())).into(),
		],
		"user has log-in",
	);
	logger.push(
		vec![
			cb::Attr::from("level", Level::WARN).into(),
			cb::Attr::from("logged", true).into(),
			cb::Attr::from("code", format!("{}:{}", file!(), line!())).into(),
			cb::Attr::from("credential_level", 'D').into(),
		],
		"token cookie is not readable",
	);
	logger.push(
		vec![
			cb::Attr::from("level", Level::ERROR).into(),
			cb::Attr::from("HTTP-code", &(404 as u16)).into(),
			cb::Attr::from("route", "/users/16472/friends").into(),
			cb::Attr::from("code", format!("{}:{}", file!(), line!())).into(),
			cb::Attr::from(
				"IPs",
				vec!["127.0.0.1", "localhost", "::1", "0:0:0:0:0:0:0:1"],
			)
			.into(),
		],
		"this is first ERROR",
	);

	println!("\t\t(see also created file {})", LOG_FILE_PATH);

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
