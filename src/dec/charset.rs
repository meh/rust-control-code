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

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Charset {
	UserPreferred,
	DEC(DEC),
	NRCS(NRCS),
	ISO(ISO),
}

impl From<DEC> for Charset {
	fn from(value: DEC) -> Self {
		Charset::DEC(value)
	}
}

impl From<NRCS> for Charset {
	fn from(value: NRCS) -> Self {
		Charset::NRCS(value)
	}
}

impl From<ISO> for Charset {
	fn from(value: ISO) -> Self {
		Charset::ISO(value)
	}
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum DEC {
	Supplemental,
	Greek,
	Hebrew,
	Turkish,
	Cyrillic,
	Graphic,
	Technical,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum NRCS {
	UK,
	French,
	FrenchCanadian,
	Norwegian,
	Finnish,
	German,
	Italian,
	Swiss,
	Swedish,
	Spanish,
	Portuguese,
	Greek,
	Hebrew,
	Turkish,
	SCS,
	Russian,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum ISO {
	Latin1,
	Latin2,
	Greek,
	Hebrew,
	Latin5,
	Cyrillic,
}
