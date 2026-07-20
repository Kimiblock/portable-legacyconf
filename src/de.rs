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
		if self.input.starts_with("true")
		|| self.input.starts_with("\"true\"") {
			self.input = &self.input["true".len()..];
			Ok(true)
		} else if self.input.starts_with("false")
		|| self.input.starts_with("\"false\""){
			self.input = &self.input["false".len()..];
			Ok(false)
		} else {
			Err(Error::ExpectedBoolean)
		}
	}
	fn parse_unsigned<T>(&mut self) -> Result<T, Error>
		where T: AddAssign<T> + MulAssign<T> + From<u8>,
	{
		let mut int = match self.next_char()? {
			ch @ '0' ..='9'	=> {
				T::from(ch as u8 - b'0')
			}
			_		=> {
				return Err(Error::ExpectedInteger);
			}
		};
		loop {
			match self.input.chars().next() {
				Some(ch @ '0' ..= '9' ) =>	{
					self.input = &self.input[1..];
					int *= T::from(10);
					int += T::from(ch as u8 - b'0')
				}
				_			=>	{
					return Ok(int);
				}
			}
		}
	}

	fn parse_signed<T>(&mut self) -> Result<T, Error>
		where T: Neg<Output = T> + AddAssign<T> + MulAssign<T> + From<i8>,
	{
		// unimplemented!()
		Err(Error::NotImplemented(String::from("signed integer")))
	}

	fn parse_string (&mut self) -> Result<&'de str, Error> {
		let has_quotes: bool;
		if self.next_char()? != '"' {
			// return Err(Error::ExpectedString)
			has_quotes = true
		} else {
			has_quotes = false
		};

		let result = {
			if has_quotes {
				self.input.find('"')
			} else {
				self.input.find("\n")
			}
		};

		match result {
			Some(v)	=> {
				let s = &self.input[..v];
				self.input = &self.input[v + 1..];
				Ok(s)
			}
			None	=> {
				Err(Error::EOF)
			}
		}
	}

	fn skip_whitespaces_and_comments(&mut self) -> Result<(), self::Error> {
		loop {
			match self.peek_char()? {
				' ' | '\n' | '\t' | '\r'	=> {
					self.next_char()?;
				}
				'#'				=> {
					loop {
						let result = self.next_char()?;
						if result == '\n' {
							break;
						}
					}
				}
				_				=> {
					return Ok(());
				}
			}
		}
	}
}

impl <'de, 'a> de::Deserializer <'de> for &'a mut Deserializer <'de> {
	type Error = Error;


	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
		where V: Visitor<'de>
	{
		self.skip_whitespaces_and_comments()?;
		match self.peek_char()? {
			'"'	=> {
				let s = self.parse_string()?;
				match s {
					"true"	=> {
						visitor.visit_bool(true)
					}
					"false"	=> {
						visitor.visit_bool(false)
					}
					_	=> {
						visitor.visit_str(s)
					}
				}
			}
			_	=> {
				return Err(Error::SyntaxError);
			}
		}
	}

	fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		self.skip_whitespaces_and_comments()?;
		visitor.visit_bool(
			self.parse_bool()?
		)
	}

	fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		self.skip_whitespaces_and_comments()?;
		visitor.visit_i8(
			self.parse_signed()?
		)
	}

	fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		self.skip_whitespaces_and_comments()?;
		visitor.visit_i16(
			self.parse_signed()?
		)
	}

	fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		self.skip_whitespaces_and_comments()?;
		visitor.visit_i32(
			self.parse_signed()?
		)
	}

	fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		self.skip_whitespaces_and_comments()?;
		visitor.visit_i64(
			self.parse_signed()?
		)
	}

	fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		self.skip_whitespaces_and_comments()?;
		visitor.visit_u8(
			self.parse_unsigned()?
		)
	}

	fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		self.skip_whitespaces_and_comments()?;
		visitor.visit_u16(
			self.parse_unsigned()?
		)
	}

	fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		self.skip_whitespaces_and_comments()?;
		visitor.visit_u32(
			self.parse_unsigned()?
		)
	}

	fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		self.skip_whitespaces_and_comments()?;
		visitor.visit_u64(
			self.parse_unsigned()?
		)
	}

	fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		Err(Error::NotImplemented(format!("f32")))
	}
	fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		Err(Error::NotImplemented(format!("f64")))
	}
	fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		Err(Error::NotImplemented(format!("char")))
	}

	fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		self.skip_whitespaces_and_comments()?;
		visitor.visit_borrowed_str(self.parse_string()?)
	}

	fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		Err(Error::NotImplemented(format!("bytes")))
	}

	fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		Err(Error::NotImplemented(format!("byte_buf")))
	}

	fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		Err(Error::NotImplemented("Option".to_string()))
	}

	fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		Err(Error::NotImplemented(format!("unit")))
	}

	fn deserialize_unit_struct<V>(
		self,
		_name: &'static str,
		_visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		Err(Error::NotImplemented(format!("unit_struct")))
	}

	fn deserialize_newtype_struct<V>(
		self,
		_name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		visitor.visit_newtype_struct(self)
	}

	fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		return Err(Error::NotImplemented(format!("seq")));
	}

	fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		return Err(Error::NotImplemented(format!("tuple")));
	}

	fn deserialize_tuple_struct<V>(
		self,
		_name: &'static str,
		_len: usize,
		_visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		return Err(Error::NotImplemented(format!("tuple_struct")));
	}

	fn deserialize_map<V>(
		self,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		visitor.visit_map(KVMapAccess::new(self.input))
	}

	fn deserialize_struct<V>(
		self,
		_name: &'static str,
		_fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		self.deserialize_map(visitor)
	}
	fn deserialize_enum<V>(
		self,
		_name: &'static str,
		_variants: &'static [&'static str],
		_visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		Err(Error::NotImplemented(format!("enum")))
	}
	fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		self.deserialize_str(visitor)
	}

	fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		self.deserialize_any(visitor)
	}

	fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>
	{
		self.deserialize_str(visitor)
	}
}

struct KVMapAccess<'a> {
	lines: std::str::Lines<'a>,
	curr_val: Option<&'a str>,
}

impl<'a> KVMapAccess<'a> {
	fn new(input: &'a str) -> Self {
		Self { lines: input.lines(), curr_val: None }
	}
}

impl <'de> MapAccess<'de> for KVMapAccess<'de> {
	type Error = Error;

	fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
	where
		K: DeserializeSeed<'de>
	{
		for line in self.lines.by_ref() {
			let trimmed = line.trim();
			if trimmed.is_empty() || trimmed.starts_with("#") {
				continue;
			};
			match trimmed.split_once("=") {
				Some(v)	=> {
					let val = v
						.1
						.trim_start_matches('"')
						.trim_end_matches('"');
					self.curr_val = Some(val);
					let mut key = Deserializer { input: v.0 };

					return seed
						.deserialize(&mut key)
						.map(Some);
				}
				None	=> {}
			};
		}
		Ok(None)
	}

	fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
	where
		V: DeserializeSeed<'de>
	{
		let val = self
			.curr_val
			.take();
		let mut val_de = {
			match val {
				Some(v)	=> Deserializer {input: v},
				None	=> {
					return Err(Error::TakeError)
				}
			}
		};
		seed.deserialize(&mut val_de)
	}
}
