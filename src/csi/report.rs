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
pub enum Report {
	CursorPosition,
}

impl Report {
	pub fn parse<'a>(value: u32) -> Result<Self, nom::Err<&'a [u8]>> {
		match value {
			6 => Ok(Report::CursorPosition),
			_ => Err(nom::Err::Code(nom::ErrorKind::Custom(9008))),
		}
	}
}

impl Into<u32> for Report {
	fn into(self) -> u32 {
		match self {
			Report::CursorPosition => 6,
		}
	}
}