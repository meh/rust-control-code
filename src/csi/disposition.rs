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
pub enum Disposition {
	ToHome,
	ToHomeWithLeader,
	Center,
	CenterWithLeader,
	ToLimit,
	ToLimitWithLeader,
	ToBoth,
}

impl Disposition {
	#[inline]
	pub fn parse<'a>(value: u32) -> Result<Self, nom::ErrorKind> {
		match value {
			0 => Ok(Disposition::ToHome),
			1 => Ok(Disposition::ToHomeWithLeader),
			2 => Ok(Disposition::Center),
			3 => Ok(Disposition::CenterWithLeader),
			4 => Ok(Disposition::ToLimit),
			5 => Ok(Disposition::ToLimitWithLeader),
			6 => Ok(Disposition::ToBoth),
			_ => Err(nom::ErrorKind::Custom(9002)),
		}
	}
}

impl Into<u32> for Disposition {
	#[inline]
	fn into(self) -> u32 {
		match self {
			Disposition::ToHome            => 0,
			Disposition::ToHomeWithLeader  => 1,
			Disposition::Center            => 2,
			Disposition::CenterWithLeader  => 3,
			Disposition::ToLimit           => 4,
			Disposition::ToLimitWithLeader => 5,
			Disposition::ToBoth            => 6,
		}
	}
}
