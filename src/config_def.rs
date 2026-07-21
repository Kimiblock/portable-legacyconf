use serde::{Deserialize, Deserializer};
use serde::de::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
	#[serde(alias = "appID")]
	app_id:		String,

	#[serde(alias = "friendlyName")]
	friendly_name:	String,

	#[serde(alias = "stateDirectory")]
	state_dir:	String,

	#[serde(alias = "launchTarget")]
	#[serde(deserialize_with = "deserialize_target")]
	target:		(String, Option<Vec<String>>),

	#[serde(alias = "bindNetwork")]
	bind_network:	bool,

	#[serde(alias = "waylandOnly")]
	wayland:	String,

	#[serde(alias = "allowGlobalShortcuts")]
	shortcuts:	bool,

	#[serde(alias = "gameMode")]
	game:		bool,

	#[serde(alias = "useZink")]
	zink:		bool,

	#[serde(alias = "qt5Compat")]
	qt5:		bool,

	#[serde(alias = "bindCameras")]
	camera:		bool,

	#[serde(alias = "bindInputDevices")]
	input_dev:	bool,

	#[serde(alias = "dbusWake")]
	tray_wake:	bool,

	#[serde(alias = "mountInfo")]
	flatpak_info:	bool,
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
