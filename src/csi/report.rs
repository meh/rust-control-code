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
	Private(u32),
}

impl Report {
	#[inline]
	pub fn parse<'a>(value: u32) -> Result<Self, nom::ErrorKind> {
		match value {
			6 => Ok(Report::CursorPosition),
			n => Ok(Report::Private(n)),
		}
	}
}

impl Into<u32> for Report {
	#[inline]
	fn into(self) -> u32 {
		match self {
			Report::CursorPosition => 6,
			Report::Private(n)     => n,
		}
	}
}
