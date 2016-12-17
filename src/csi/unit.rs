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
pub enum Unit {
	Character,
	Millimeter,
	ComputerDecipoint,
	Decidot,
	Mil,
	BasicMeasuringUnit,
	Micrometer,
	Pixel,
	Decipoint,
}

impl Unit {
	#[inline]
	pub fn parse<'a>(value: u32) -> Result<Self, nom::ErrorKind> {
		match value {
			0 => Ok(Unit::Character),
			1 => Ok(Unit::Millimeter),
			2 => Ok(Unit::ComputerDecipoint),
			3 => Ok(Unit::Decidot),
			4 => Ok(Unit::Mil),
			5 => Ok(Unit::BasicMeasuringUnit),
			6 => Ok(Unit::Micrometer),
			7 => Ok(Unit::Pixel),
			8 => Ok(Unit::Decipoint),
			_ => Err(nom::ErrorKind::Custom(9002)),
		}
	}
}

impl Into<u32> for Unit {
	#[inline]
	fn into(self) -> u32 {
		match self {
			Unit::Character          => 0,
			Unit::Millimeter         => 1,
			Unit::ComputerDecipoint  => 2,
			Unit::Decidot            => 3,
			Unit::Mil                => 4,
			Unit::BasicMeasuringUnit => 5,
			Unit::Micrometer         => 6,
			Unit::Pixel              => 7,
			Unit::Decipoint          => 8,
		}
	}
}
