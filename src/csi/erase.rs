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
pub enum Erase {
	ToEnd,
	ToStart,
	All,
}

impl Erase {
	pub fn parse<'a>(value: u32) -> Result<Self, nom::ErrorKind> {
		match value {
			0 => Ok(Erase::ToEnd),
			1 => Ok(Erase::ToStart),
			2 => Ok(Erase::All),
			_ => Err(nom::ErrorKind::Custom(9002)),
		}
	}
}

impl Into<u32> for Erase {
	fn into(self) -> u32 {
		match self {
			Erase::ToEnd   => 0,
			Erase::ToStart => 1,
			Erase::All     => 2,
		}
	}
}
