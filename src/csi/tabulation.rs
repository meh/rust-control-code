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
pub enum Tabulation {
	Character,
	Line,
	Characters,
	AllCharacters,
	AllLines,
	All,
}

impl Tabulation {
	pub fn parse<'a>(value: u32) -> Result<Self, nom::Err<&'a [u8]>> {
		match value {
			0 => Ok(Tabulation::Character),
			1 => Ok(Tabulation::Line),
			2 => Ok(Tabulation::Characters),
			3 => Ok(Tabulation::AllCharacters),
			4 => Ok(Tabulation::AllLines),
			5 => Ok(Tabulation::All),
			_ => Err(nom::Err::Code(nom::ErrorKind::Custom(9003))),
		}
	}
}

impl Into<u32> for Tabulation {
	fn into(self) -> u32 {
		match self {
			Tabulation::Character     => 0,
			Tabulation::Line          => 1,
			Tabulation::Characters    => 2,
			Tabulation::AllCharacters => 3,
			Tabulation::AllLines      => 4,
			Tabulation::All           => 5,
		}
	}
}
