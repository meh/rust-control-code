//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

use nom;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Combination {
	Next,
	Start,
	End,
}

impl Combination {
	pub fn parse<'a>(value: u32) -> Result<Self, nom::Err<&'a [u8]>> {
		match value {
			0 => Ok(Combination::Next),
			1 => Ok(Combination::Start),
			2 => Ok(Combination::End),
			_ => Err(nom::Err::Code(nom::ErrorKind::Custom(9005))),
		}
	}
}

impl Into<u32> for Combination {
	fn into(self) -> u32 {
		match self {
			Combination::Next  => 0,
			Combination::Start => 1,
			Combination::End   => 2,
		}
	}
}
