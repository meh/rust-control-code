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

use std::io::{self, Write};
use {Format};

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum DEC {
	AlignmentTest,
	SaveCursor,
	RestoreCursor,
	ApplicationKeypad(bool),
	BackIndex,
	ForwardIndex,
	SaveCursorPosition,
	RestoreCursorPosition,
	SelectCharset(u8, Charset),
	Set(Vec<Mode>),
	Reset(Vec<Mode>),
	DeleteColumn(u32),
	InsertColumn(u32),
	Double(Half),
	DoubleWidth,
	SingleWidth,
	CursorStyle(u8),
	SoftReset,

	ScrollRegion {
		top:    u32,
		bottom: Option<u32>,
	}
}

use self::DEC::*;

impl Format for DEC {
	fn fmt<W: Write>(&self, mut f: W, wide: bool) -> io::Result<()> {
		Ok(())
	}
}

mod mode;
pub use self::mode::Mode;

mod half;
pub use self::half::Half;

pub mod charset;
pub use self::charset::Charset;

named!(pub parse<DEC>,
	chain!(tag!(b"\x1B") ~
		res: switch!(take!(1),
			b"#" => switch!(take!(1),
				b"3" => call!(DECDHLT) |
				b"4" => call!(DECDHLB) |
				b"5" => call!(DECSWL)  |
				b"6" => call!(DECDWL)  |
				b"8" => call!(DECALN)) |

			b"(" => map!(SCS, |c| SelectCharset(0, c)) |
			b")" => map!(SCS, |c| SelectCharset(1, c)) |
			b"*" => map!(SCS, |c| SelectCharset(2, c)) |
			b"+" => map!(SCS, |c| SelectCharset(3, c)) |
			b"-" => map!(SCS, |c| SelectCharset(1, c)) |
			b"." => map!(SCS, |c| SelectCharset(2, c)) |
			b"/" => map!(SCS, |c| SelectCharset(3, c)) |

			b"6" => call!(DECBI)   |
			b"7" => call!(DECSC)   |
			b"8" => call!(DECRC)   |
			b"9" => call!(DECFI)   |
			b"=" => call!(DECKPAM) |
			b">" => call!(DECKPNM)),

	|| res));

named!(DECALN<DEC>,
	value!(AlignmentTest));

named!(DECBI<DEC>,
	value!(BackIndex));

named!(DECSC<DEC>,
	value!(SaveCursor));

named!(DECRC<DEC>,
	value!(RestoreCursor));

named!(DECFI<DEC>,
	value!(ForwardIndex));

named!(DECKPAM<DEC>,
	value!(ApplicationKeypad(true)));

named!(DECKPNM<DEC>,
	value!(ApplicationKeypad(false)));

named!(DECDHLT<DEC>,
	value!(Double(Half::Top)));

named!(DECDHLB<DEC>,
	value!(Double(Half::Bottom)));

named!(DECSWL<DEC>,
	value!(SingleWidth));

named!(DECDWL<DEC>,
	value!(DoubleWidth));

named!(SCS<Charset>,
	switch!(take!(1),
		b"<" => value!(Charset::UserPreferred) |

		b"0" => value!(charset::DEC::Graphic.into()) |
		b"9" => value!(charset::NRCS::FrenchCanadian.into()) |
		b"Q" => value!(charset::NRCS::FrenchCanadian.into()) |
		b"`" => value!(charset::NRCS::Norwegian.into()) |
		b"E" => value!(charset::NRCS::Norwegian.into()) |
		b"6" => value!(charset::NRCS::Norwegian.into()) |
		b"5" => value!(charset::NRCS::Finnish.into()) |
		b"C" => value!(charset::NRCS::Finnish.into()) |
		b"7" => value!(charset::NRCS::Swedish.into()) |

		b"A" => value!(charset::ISO::Latin1.into())   |
		b"B" => value!(charset::ISO::Latin2.into())   |
		b"F" => value!(charset::ISO::Greek.into())    |
		b"H" => value!(charset::ISO::Hebrew.into())   |
		b"M" => value!(charset::ISO::Latin5.into())   |
		b"L" => value!(charset::ISO::Cyrillic.into()) |

		b"\"" => switch!(take!(1),
			b"?" => value!(charset::DEC::Greek.into()) |
			b"4" => value!(charset::DEC::Hebrew.into()) |
			b">" => value!(charset::NRCS::Greek.into())) |

		b"&" => switch!(take!(1),
			b"4" => value!(charset::DEC::Cyrillic.into()) |
			b"5" => value!(charset::NRCS::Russian.into())) |

		b"%" => switch!(take!(1),
			b"5" => value!(charset::DEC::Supplemental.into()) |
			b"0" => value!(charset::DEC::Turkish.into()) |
			b"6" => value!(charset::NRCS::Portuguese.into()) |
			b"=" => value!(charset::NRCS::Hebrew.into()) |
			b"2" => value!(charset::NRCS::Turkish.into()) |
			b"3" => value!(charset::NRCS::SCS.into()))));

pub mod csi;

pub mod shim {
	pub use super::DEC as T;
	pub use super::DEC::*;
	pub use super::parse;
	pub use super::csi as CSI;
	pub use super::{Mode, Charset};
	pub use super::charset;
}
