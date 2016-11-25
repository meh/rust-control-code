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
pub enum Copy {
	ToPrimary,
	FromPrimary,
	ToSecondary,
	FromSecondary,
	StopPrimary,
	StartPrimary,
	StopSecondary,
	StartSecondary,
}

impl Copy {
	pub fn parse<'a>(value: u32) -> Result<Self, nom::ErrorKind> {
		match value {
			0 => Ok(Copy::ToPrimary),
			1 => Ok(Copy::FromPrimary),
			2 => Ok(Copy::ToSecondary),
			3 => Ok(Copy::FromSecondary),
			4 => Ok(Copy::StopPrimary),
			5 => Ok(Copy::StartPrimary),
			6 => Ok(Copy::StopSecondary),
			7 => Ok(Copy::StartSecondary),
			_ => Err(nom::ErrorKind::Custom(9005)),
		}
	}
}

impl Into<u32> for Copy {
	fn into(self) -> u32 {
		match self {
			Copy::ToPrimary      => 0,
			Copy::FromPrimary    => 1,
			Copy::ToSecondary    => 2,
			Copy::FromSecondary  => 3,
			Copy::StopPrimary    => 4,
			Copy::StartPrimary   => 5,
			Copy::StopSecondary  => 6,
			Copy::StartSecondary => 7,
		}
	}
}
