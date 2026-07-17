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
