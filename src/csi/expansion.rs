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
pub enum Expansion {
	Normal,
	Expanded,
	Condensed,
}

impl Expansion {
	pub fn parse<'a>(value: u32) -> Result<Self, nom::Err<&'a [u8]>> {
		match value {
			0 => Ok(Expansion::Normal),
			1 => Ok(Expansion::Expanded),
			2 => Ok(Expansion::Condensed),
			_ => Err(nom::Err::Code(nom::ErrorKind::Custom(9002))),
		}
	}
}

impl Into<u32> for Expansion {
	fn into(self) -> u32 {
		match self {
			Expansion::Normal    => 0,
			Expansion::Expanded  => 1,
			Expansion::Condensed => 2,
		}
	}
}
