// Constants

const SERVICES: &'static str = include_str!("../data/services.csv");

const SERVICES_DEFAULT: &'static [&str] = &[
	"ClientState",
	"CommandManager",
	"DataManager",
	"ObjectTable",
	"SigScanner"
];

// Parser

pub struct Service {
	pub name: String,
	pub namespace: String,
	pub enabled: bool
}

impl Service {
	pub fn get_all() -> Vec<Self> {
		SERVICES.split("\n")
		.map(|ln| {
			let split: Vec<&str> = ln.split(",").collect();
			let name = split[0].to_string();
			let namespace = split[1].to_string();

			let enabled = SERVICES_DEFAULT
			.into_iter()
			.position(|x| x.to_string() == name)
			.is_some();

			return Self {
				name,
				namespace,
				enabled
			};
		}).collect()
	}
}