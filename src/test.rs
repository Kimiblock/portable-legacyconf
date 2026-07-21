use serde::{Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TestStruct {
	#[serde(alias = "appID")]
	app_id:		String,

	#[serde(alias = "friendlyName")]
	friendly_name:	String,

	#[serde(alias = "stateDirectory")]
	state_dir:	String,

	#[serde(alias = "launchTarget")]
	target:		String,

	#[serde(alias = "bindNetwork")]
	bind_network:	bool,

	#[serde(alias = "waylandOnly")]
	wayland:	String,

	#[serde(alias = "gameMode")]
	game:		bool,
}

#[cfg(test)]
mod tests {
	//use super::*;
	#[test]
	fn decode_legacy_conf() -> Result<(), crate::Error> {
		let data = r#"
#!/usr/bin/bash

########## Section: General ##########

# This is your Application ID, avoid conflict
appID="cafe.avery.Delfin"

# This is a friendly name of an application. It should only contain ASCII characters and not spaces.
friendlyName="Delfin"

# This is the state directory of your application, which is located under "XDG_DATA_HOME"
stateDirectory="Delfin_Data"

# This is the target executable to launch
launchTarget="delfin"

# Enables Network access within the sandbox. Defaults to true.
bindNetwork="true"

# Takes a boolean value or string "adaptive". When true, do not process XAuth files and forces wayland input method. "adaptive" enables this option on Wayland and vice versa. Generally this should be adaptive when possible.
waylandOnly="adaptive"

# Expose all GPUs. Takes boolean value, usually used in games.
gameMode="false"

# If not empty, will own the org.mpris.MediaPlayer2.mprisName bus name instead of "org.mpris.MediaPlayer2.${appID##*.}"
#mprisName="mpv"

########## Section: Portals ##########

# If true, allows the sandboxed application to use the org.freedesktop.portal.Inhibit interface to inhibit certain actions, like suspend and logout
allowInhibit="true"

# If true, allow apps to register Global Shortcuts via the Portal.
allowGlobalShortcuts="false"
		"#;

		println!("Testing against data: {data}");

		let decoded: crate::test::TestStruct
			= crate::from_str(data)?;
		println!("Decoded legacy config: {decoded:#?}");
		assert_eq!(decoded.app_id, "cafe.avery.Delfin");
		assert_eq!(decoded.friendly_name, "Delfin");
		assert_eq!(decoded.state_dir, "Delfin_Data");
		assert_eq!(decoded.target, "delfin");
		assert_eq!(decoded.bind_network, true);
		assert_eq!(decoded.wayland, "adaptive");
		assert_eq!(decoded.game, false);
		Ok(())
	}
}
