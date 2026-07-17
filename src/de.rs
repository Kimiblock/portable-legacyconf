use std::ops::{AddAssign, MulAssign, Neg};

use serde::Deserialize;
use serde::de::{
	self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess,
	VariantAccess, Visitor,
};


use crate::error::Error;

pub struct Deserializer<'de> {
	input: &'de str,
}

impl <'de> Deserializer <'de> {
	pub fn from_str(input: &'de str) -> Self {
		Deserializer { input }
	}
}

pub fn from_str<'a, T> (s: &'a str) -> Result<T, Error>
	where T: Deserialize<'a>,
{
	let mut deserializer = Deserializer::from_str(s);
	let t = T::deserialize(&mut deserializer)?;
	if deserializer.input.is_empty() {
		Ok(t)
	} else {
		Err(Error::TrailingCharacters)
	}
}

impl <'de> Deserializer <'de> {
	fn peek_char(&mut self) -> Result<char, Error> {
		self.input.chars().next().ok_or(Error::EOF)
	}
	fn next_char(&mut self) -> Result<char, Error> {
		let cha = self.peek_char()?;
		self.input = &self.input[cha.len_utf8()..];
		Ok(cha)
	}
	fn parse_bool(&mut self) -> Result<bool, Error> {
		if self.input.starts_with("true") {
			self.input = &self.input["true".len()..];
			Ok(true)
		} else if self.input.starts_with("false") {
			self.input = &self.input["false".len()..];
			Ok(false)
		} else {
			Err(Error::ExpectedBoolean)
		}
	}
}
