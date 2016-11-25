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
pub enum Qualification {
	UnprotectedUnguarded,
	ProtectedGuarded,
	GraphicCharacterInput,
	NumericInput,
	AlphabeticInput,
	AlignLast,
	ZeroFill,
	FieldStart,
	ProtectedUnguarded,
	SpaceFill,
	AlignFirst,
	Reverse,
}

impl Qualification {
	pub fn parse<'a>(value: u32) -> Result<Self, nom::ErrorKind> {
		match value {
			0  => Ok(Qualification::UnprotectedUnguarded),
			1  => Ok(Qualification::ProtectedGuarded),
			2  => Ok(Qualification::GraphicCharacterInput),
			3  => Ok(Qualification::NumericInput),
			4  => Ok(Qualification::AlphabeticInput),
			5  => Ok(Qualification::AlignLast),
			6  => Ok(Qualification::ZeroFill),
			7  => Ok(Qualification::FieldStart),
			8  => Ok(Qualification::ProtectedUnguarded),
			9  => Ok(Qualification::SpaceFill),
			10 => Ok(Qualification::AlignFirst),
			11 => Ok(Qualification::Reverse),
			_  => Err(nom::ErrorKind::Custom(9004)),
		}
	}
}

impl Into<u32> for Qualification {
	fn into(self) -> u32 {
		match self {
			Qualification::UnprotectedUnguarded  => 0,
			Qualification::ProtectedGuarded      => 1,
			Qualification::GraphicCharacterInput => 2,
			Qualification::NumericInput          => 3,
			Qualification::AlphabeticInput       => 4,
			Qualification::AlignLast             => 5,
			Qualification::ZeroFill              => 6,
			Qualification::FieldStart            => 7,
			Qualification::ProtectedUnguarded    => 8,
			Qualification::SpaceFill             => 9,
			Qualification::AlignFirst            => 10,
			Qualification::Reverse               => 11,
		}
	}
}
