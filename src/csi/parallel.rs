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
pub enum Parallel {
	End,
	Start,
	StartSupplementary,
	StartPhoneticJapanese,
	StartPhoneticChinese,
	StopPhonetic,
}

impl Parallel {
	pub fn parse<'a>(value: u32) -> Result<Self, nom::Err<&'a [u8]>> {
		match value {
			0 => Ok(Parallel::End),
			1 => Ok(Parallel::Start),
			2 => Ok(Parallel::StartSupplementary),
			3 => Ok(Parallel::StartPhoneticJapanese),
			4 => Ok(Parallel::StartPhoneticChinese),
			5 => Ok(Parallel::StopPhonetic),
			_ => Err(nom::Err::Code(nom::ErrorKind::Custom(9002))),
		}
	}
}


impl Into<u32> for Parallel {
	fn into(self) -> u32 {
		match self {
			Parallel::End                   => 0,
			Parallel::Start                 => 1,
			Parallel::StartSupplementary    => 2,
			Parallel::StartPhoneticJapanese => 3,
			Parallel::StartPhoneticChinese  => 4,
			Parallel::StopPhonetic          => 5,
		}
	}
}
