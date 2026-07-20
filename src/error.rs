use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {
	#[error("failed reading configuration: {0:#?}")]
	Message(String),

	#[error("trailing characters")]
	TrailingCharacters,

	#[error("end of file")]
	EOF,

	#[error("expected boolean")]
	ExpectedBoolean,

	#[error("expected integer")]
	ExpectedInteger,

	#[error("not implemented: {0}")]
	NotImplemented(String),

	#[error("expected String with double quote")]
	ExpectedString,

	#[error("syntax error")]
	SyntaxError,

	#[error("incomplete string quote")]
	IncompleteQuoteError,

	#[error(".take() failed")]
	TakeError,
}

impl serde::de::Error for Error {
	fn custom<T>(msg: T) -> Self
		where T: std::fmt::Display {
		Error::Message(msg.to_string())
	}
}
