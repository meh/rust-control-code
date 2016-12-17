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
pub enum Mode {
	GuardedAreaTransfer,
	KeyboardLock,
	ControlRepresentation,
	InsertionReplacement,
	StatusReportTransfer,
	Erasure,
	LineEditing,
	BidirectionalSupport,
	DeviceComponentSelect,
	CharacterEditing,
	PositioningUnit,
	SendReceive,
	FormatEffectorAction,
	FormatEffectorTransfer,
	MultipleAreaTransfer,
	TransferTermination,
	SelectedAreaTransfer,
	TabulationStop,
	LineFeed,
	GraphicRenditionCombination,
	ZeroDefault,
}

impl Mode {
	#[inline]
	pub fn parse<'a>(value: u32) -> Result<Self, nom::ErrorKind> {
		match value {
			1  => Ok(Mode::GuardedAreaTransfer),
			2  => Ok(Mode::KeyboardLock),
			3  => Ok(Mode::ControlRepresentation),
			4  => Ok(Mode::InsertionReplacement),
			5  => Ok(Mode::StatusReportTransfer),
			6  => Ok(Mode::Erasure),
			7  => Ok(Mode::LineEditing),
			8  => Ok(Mode::BidirectionalSupport),
			9  => Ok(Mode::DeviceComponentSelect),
			10 => Ok(Mode::CharacterEditing),
			11 => Ok(Mode::PositioningUnit),
			12 => Ok(Mode::SendReceive),
			13 => Ok(Mode::FormatEffectorAction),
			14 => Ok(Mode::FormatEffectorTransfer),
			15 => Ok(Mode::MultipleAreaTransfer),
			16 => Ok(Mode::TransferTermination),
			17 => Ok(Mode::SelectedAreaTransfer),
			18 => Ok(Mode::TabulationStop),
			20 => Ok(Mode::LineFeed),
			21 => Ok(Mode::GraphicRenditionCombination),
			22 => Ok(Mode::ZeroDefault),
			_  => Err(nom::ErrorKind::Custom(9004)),
		}
	}
}

impl Into<u32> for Mode {
	#[inline]
	fn into(self) -> u32 {
		match self {
			Mode::GuardedAreaTransfer         => 1,
			Mode::KeyboardLock                => 2,
			Mode::ControlRepresentation       => 3,
			Mode::InsertionReplacement        => 4,
			Mode::StatusReportTransfer        => 5,
			Mode::Erasure                     => 6,
			Mode::LineEditing                 => 7,
			Mode::BidirectionalSupport        => 8,
			Mode::DeviceComponentSelect       => 9,
			Mode::CharacterEditing            => 10,
			Mode::PositioningUnit             => 11,
			Mode::SendReceive                 => 12,
			Mode::FormatEffectorAction        => 13,
			Mode::FormatEffectorTransfer      => 14,
			Mode::MultipleAreaTransfer        => 15,
			Mode::TransferTermination         => 16,
			Mode::SelectedAreaTransfer        => 17,
			Mode::TabulationStop              => 18,
			Mode::LineFeed                    => 20,
			Mode::GraphicRenditionCombination => 21,
			Mode::ZeroDefault                 => 22,
		}
	}
}
