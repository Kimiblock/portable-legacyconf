use serde::{Deserialize, Deserializer};
use serde::de::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
	#[serde(alias = "appID")]
	pub app_id:		String,

	#[serde(alias = "friendlyName")]
	pub friendly_name:	String,

	#[serde(alias = "stateDirectory")]
	pub state_dir:	String,

	#[serde(alias = "launchTarget")]
	#[serde(deserialize_with = "deserialize_target")]
	pub target:		(String, Option<Vec<String>>),

	#[serde(alias = "bindNetwork")]
	pub bind_network:	bool,

	#[serde(alias = "waylandOnly")]
	#[serde(deserialize_with = "deserialize_wayland")]
	pub wayland:		bool,

	#[serde(alias = "allowGlobalShortcuts")]
	pub shortcuts:	bool,

	#[serde(alias = "gameMode")]
	pub game:		bool,

	#[serde(alias = "useZink")]
	pub zink:		bool,

	#[serde(alias = "qt5Compat")]
	pub qt5:		bool,

	#[serde(alias = "bindCameras")]
	pub camera:		bool,

	#[serde(alias = "bindInputDevices")]
	pub input_dev:	bool,

	#[serde(alias = "dbusWake")]
	pub tray_wake:	bool,

	#[serde(alias = "mountInfo")]
	pub flatpak_info:	bool,
}

fn deserialize_wayland <'de, D> (deserializer: D) -> Result<bool, D::Error>
	where
		D: Deserializer<'de>
{
	let raw = String::deserialize(deserializer)?;
	match raw.as_str() {
		"adaptive" | "true"	=> {
			Ok(true)
		}
		"false"			=> {
			Ok(false)
		}
		_v			=> {
			Err(D::Error::custom("invalid waylandOnly value: {_v}"))
		}
	}
}

// Returns the Target and arguments, separated with spaces
fn deserialize_target <'de, D> (deserializer: D) -> Result<(String, Option<Vec<String>>), D::Error>
	where
		D: Deserializer<'de>,
{
	let deserialize = String::deserialize(deserializer)?;
	let raw: Vec<&str> = {
		deserialize.split(" ").collect()
	};

	match raw.len() {
		0	=> {
			return Err(D::Error::custom("target not found"));
		}
		1	=> {
			let target = raw.into_iter().next();
			match target {
				Some(v)	=> {
					return Ok((v.into(), None));
				}
				None	=> {
					return Err(D::Error::custom("error decoding target"));
				}
			}
		}
		_	=> {
			let mut iter = raw.into_iter();
			let target = {
				let first = iter.next();
				match first {
					Some(v)	=> {
						v
					}
					None	=> {
						return Err(D::Error::custom("error decoding target"));
					}
				}
			};
			let mut args: Vec<String> = vec![];
			for arg in iter {
				args.push(arg.into());
			};
			return Ok((target.into(), Some(args)));
		}
	}
}
