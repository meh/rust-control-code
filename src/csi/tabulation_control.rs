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
pub enum TabulationControl {
	Character,
	Line,
	ClearCharacter,
	ClearLine,
	ClearCharacters,
	ClearAllCharacters,
	ClearAllLines,
}

impl TabulationControl {
	pub fn parse<'a>(value: u32) -> Result<Self, nom::ErrorKind> {
		match value {
			0 => Ok(TabulationControl::Character),
			1 => Ok(TabulationControl::Line),
			2 => Ok(TabulationControl::ClearCharacter),
			3 => Ok(TabulationControl::ClearLine),
			4 => Ok(TabulationControl::ClearCharacters),
			5 => Ok(TabulationControl::ClearAllCharacters),
			6 => Ok(TabulationControl::ClearAllLines),
			_ => Err(nom::ErrorKind::Custom(9003)),
		}
	}
}

impl Into<u32> for TabulationControl {
	fn into(self) -> u32 {
		match self {
			TabulationControl::Character          => 0,
			TabulationControl::Line               => 1,
			TabulationControl::ClearCharacter     => 2,
			TabulationControl::ClearLine          => 3,
			TabulationControl::ClearCharacters    => 4,
			TabulationControl::ClearAllCharacters => 5,
			TabulationControl::ClearAllLines      => 6,
		}
	}
}
