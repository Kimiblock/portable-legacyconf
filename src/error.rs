use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {
	#[error("failed reading configuration: {0:#?}")]
	Message(String)
}

impl serde::de::Error for Error {
	fn custom<T>(msg: T) -> Self
		where T: std::fmt::Display {
		Error::Message(msg.to_string())
	}
}
