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
use std::str;
use smallvec::SmallVec;
use nom;
use {Format, CSI};

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum DEC<'a> {
	AlignmentTest,
	SaveCursor,
	RestoreCursor,
	ApplicationKeypad(bool),
	BackIndex,
	ForwardIndex,
	SaveCursorPosition,
	RestoreCursorPosition,
	SelectCharset(u8, Charset),
	Set(SmallVec<[Mode; CSI::SIZE]>),
	Reset(SmallVec<[Mode; CSI::SIZE]>),
	DeleteColumn(u32),
	InsertColumn(u32),
	Double(Half),
	DoubleWidth,
	SingleWidth,
	CursorStyle(u8),
	SoftReset,
	ResetInitial,
	SevenBits,
	EightBits,
	DefineFunctionKey(u8, &'a str),
	Unicode(bool),
	Sixel(sixel::Header, &'a [u8]),

	ScrollRegion {
		top:    u32,
		bottom: Option<u32>,
	},
}

use self::DEC::*;

impl<'a> Format for DEC<'a> {
	fn fmt<W: Write>(&self, mut f: W, wide: bool) -> io::Result<()> {
		macro_rules! write {
			(csi $($value:tt)*) => (
				try!(CSI::$($value)*.fmt(f.by_ref(), wide));
			);

			(fmt $($value:tt)*) => (
				try!($($value)*.fmt(f.by_ref(), wide));
			);

			(code $code:expr) => (
				if wide {
					try!(f.write_all(&[0x1B, $code - 0x40]));
				}
				else {
					try!(f.write_all(&[$code]));
				}
			);

			($string:expr) => (
				try!(f.write_all($string));
			);
		}

		match *self {
			AlignmentTest =>
				write!(b"\x1B#8"),

			SaveCursor =>
				write!(b"\x1B7"),

			RestoreCursor =>
				write!(b"\x1B8"),

			ApplicationKeypad(true) =>
				write!(b"\x1B="),

			ApplicationKeypad(false) =>
				write!(b"\x1B>"),

			BackIndex =>
				write!(b"\x1B6"),

			ForwardIndex =>
				write!(b"\x1B9"),

			SaveCursorPosition =>
				write!(csi Unknown(b's', None, small_vec![])),

			RestoreCursorPosition =>
				write!(csi Unknown(b'u', None, small_vec![])),

			SelectCharset(group, charset) => {
				use self::charset::*;

				write!(b"\x1B");

				match group {
					0 => write!(b"("),
					1 => write!(b")"),
					2 => write!(b"*"),
					3 => write!(b"+"),
					_ => unreachable!(),
				}

				try!(f.write_all(match charset {
					Charset::UserPreferred => b"<",

					Charset::DEC(DEC::Supplemental) => b"%5",
					Charset::DEC(DEC::Greek)        => b"\"?",
					Charset::DEC(DEC::Hebrew)       => b"\"4",
					Charset::DEC(DEC::Turkish)      => b"%0",
					Charset::DEC(DEC::Cyrillic)     => b"&4",
					Charset::DEC(DEC::Graphic)      => b"0",
					Charset::DEC(DEC::Technical)    => b">",

					Charset::NRCS(NRCS::UK)             => b"A",
					Charset::NRCS(NRCS::French)         => b"R",
					Charset::NRCS(NRCS::FrenchCanadian) => b"Q",
					Charset::NRCS(NRCS::Norwegian)      => b"E",
					Charset::NRCS(NRCS::Finnish)        => b"C",
					Charset::NRCS(NRCS::German)         => b"K",
					Charset::NRCS(NRCS::Italian)        => b"Y",
					Charset::NRCS(NRCS::Swiss)          => b"=",
					Charset::NRCS(NRCS::Swedish)        => b"7",
					Charset::NRCS(NRCS::Spanish)        => b"Z",
					Charset::NRCS(NRCS::Portuguese)     => b"%6",
					Charset::NRCS(NRCS::Greek)          => b"\">",
					Charset::NRCS(NRCS::Hebrew)         => b"%=",
					Charset::NRCS(NRCS::Turkish)        => b"%2",
					Charset::NRCS(NRCS::SCS)            => b"%3",
					Charset::NRCS(NRCS::Russian)        => b"&5",

					Charset::ISO(ISO::Latin1)   => b"A",
					Charset::ISO(ISO::Latin2)   => b"B",
					Charset::ISO(ISO::Greek)    => b"F",
					Charset::ISO(ISO::Hebrew)   => b"H",
					Charset::ISO(ISO::Latin5)   => b"M",
					Charset::ISO(ISO::Cyrillic) => b"L",
				}));
			}

			Set(ref modes) =>
				write!(csi Private(b'h', None, modes.iter().map(|&m| Some(m.into())).collect())),

			Reset(ref modes) =>
				write!(csi Private(b'l', None, modes.iter().map(|&m| Some(m.into())).collect())),

			DeleteColumn(n) =>
				write!(csi Private(b'~', Some(b'\''), small_vec![Some(n)])),

			InsertColumn(n) =>
				write!(csi Private(b'}', Some(b'\''), small_vec![Some(n)])),

			Double(Half::Top) =>
				write!(b"\x1B#3"),

			Double(Half::Bottom) =>
				write!(b"\x1B#4"),

			DoubleWidth =>
				write!(b"\x1B#6"),

			SingleWidth =>
				write!(b"\x1B#5"),

			CursorStyle(id) =>
				write!(csi Unknown(b'q', Some(b' '), small_vec![Some(id as u32)])),

			SoftReset =>
				write!(csi Unknown(b'p', Some(b'!'), small_vec![])),

			ResetInitial =>
				write!(b"\x1Bc"),

			SevenBits =>
				write!(b"\x1B F"),

			EightBits =>
				write!(b"\x1B G"),

			DefineFunctionKey(key, string) => {
				write!(&[0x1B, b'Q', key + b'0' - 1]);
				write!(b"'");
				write!(string.as_bytes());
				write!(b"'");
			}

			Unicode(value) => {
				write!(b"\x1B%");
				write!(&[if value { b'G' } else { b'@' }]);
			}

			ScrollRegion { top, bottom } =>
				write!(csi Unknown(b'r', None, small_vec![Some(top + 1), bottom.map(|v| v + 1)])),

			Sixel(header, content) => {
				write!(code 0x90);
				write!(fmt header);
				write!(content);
				write!(code 0x9C);
			}
		}

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
	do_parse!(tag!(b"\x1B") >>
		res: switch!(take!(1),
			b"#" => switch!(take!(1),
				b"3" => call!(DECDHLT) |
				b"4" => call!(DECDHLB) |
				b"5" => call!(DECSWL)  |
				b"6" => call!(DECDWL)  |
				b"8" => call!(DECALN)) |

			b" " => switch!(take!(1),
				b"F" => call!(S7C1T)  |
				b"G" => call!(S8C1T)) |

			b"%" => switch!(take!(1),
				b"G" => value!(Unicode(true))   |
				b"@" => value!(Unicode(false))) |

			b"Q" => call!(SCODFK) |

			b"(" => map!(SCS, |c| SelectCharset(0, c)) |
			b")" => map!(SCS, |c| SelectCharset(1, c)) |
			b"*" => map!(SCS, |c| SelectCharset(2, c)) |
			b"+" => map!(SCS, |c| SelectCharset(3, c)) |
			b"-" => map!(SCS, |c| SelectCharset(1, c)) |
			b"." => map!(SCS, |c| SelectCharset(2, c)) |
			b"/" => map!(SCS, |c| SelectCharset(3, c)) |

			b"c" => call!(RIS) |

			b"6" => call!(DECBI)   |
			b"7" => call!(DECSC)   |
			b"8" => call!(DECRC)   |
			b"9" => call!(DECFI)   |
			b"=" => call!(DECKPAM) |
			b">" => call!(DECKPNM)) >>

	(res)));

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

named!(RIS<DEC>,
	value!(ResetInitial));

named!(S7C1T<DEC>,
	value!(SevenBits));

named!(S8C1T<DEC>,
	value!(EightBits));

named!(SCODFK<DEC>,
	do_parse!(
		key:       is_key >>
		delimiter: take!(1) >>
		string:    take_until!(delimiter) >>
		tag!(delimiter) >>

		(DefineFunctionKey(key, unsafe { str::from_utf8_unchecked(string) }))));

fn is_key(i: &[u8]) -> nom::IResult<&[u8], u8> {
	if i.is_empty() {
		return nom::IResult::Incomplete(nom::Needed::Unknown);
	}

	let key = i[0];

	if key >= b'0' && key <= b'k' {
		nom::IResult::Done(&i[1 ..], key - b'0' + 1)
	}
	else {
		nom::IResult::Error(nom::ErrorKind::Custom(0))
	}
}

named!(SCS<Charset>,
	switch!(take!(1),
		b"<" => value!(Charset::UserPreferred) |

		b">" => value!(charset::DEC::Technical.into())       |
		b"0" => value!(charset::DEC::Graphic.into())         |
		b"9" => value!(charset::NRCS::FrenchCanadian.into()) |
		b"Q" => value!(charset::NRCS::FrenchCanadian.into()) |
		b"`" => value!(charset::NRCS::Norwegian.into())      |
		b"E" => value!(charset::NRCS::Norwegian.into())      |
		b"6" => value!(charset::NRCS::Norwegian.into())      |
		b"5" => value!(charset::NRCS::Finnish.into())        |
		b"C" => value!(charset::NRCS::Finnish.into())        |
		b"7" => value!(charset::NRCS::Swedish.into())        |
		b"R" => value!(charset::NRCS::French.into())         |
		b"K" => value!(charset::NRCS::German.into())         |
		b"Y" => value!(charset::NRCS::Italian.into())        |
		b"=" => value!(charset::NRCS::Swiss.into())          |
		b"Z" => value!(charset::NRCS::Spanish.into())        |

		b"A" => value!(charset::ISO::Latin1.into())   |
		b"B" => value!(charset::ISO::Latin2.into())   |
		b"F" => value!(charset::ISO::Greek.into())    |
		b"H" => value!(charset::ISO::Hebrew.into())   |
		b"M" => value!(charset::ISO::Latin5.into())   |
		b"L" => value!(charset::ISO::Cyrillic.into()) |

		b"\"" => switch!(take!(1),
			b"?" => value!(charset::DEC::Greek.into())   |
			b"4" => value!(charset::DEC::Hebrew.into())  |
			b">" => value!(charset::NRCS::Greek.into())) |

		b"&" => switch!(take!(1),
			b"4" => value!(charset::DEC::Cyrillic.into())  |
			b"5" => value!(charset::NRCS::Russian.into())) |

		b"%" => switch!(take!(1),
			b"5" => value!(charset::DEC::Supplemental.into()) |
			b"0" => value!(charset::DEC::Turkish.into())      |
			b"6" => value!(charset::NRCS::Portuguese.into())  |
			b"=" => value!(charset::NRCS::Hebrew.into())      |
			b"2" => value!(charset::NRCS::Turkish.into())     |
			b"3" => value!(charset::NRCS::SCS.into()))));

pub mod csi;
pub mod sixel;

pub mod shim {
	pub use super::DEC as T;
	pub use super::DEC::*;
	pub use super::parse;
	pub use super::csi as CSI;
	pub use super::sixel::shim as SIXEL;
	pub use super::{Mode, Charset, Half};
	pub use super::charset;
}

#[cfg(test)]
mod test {
	mod parse {
		use {Control, DEC, parse};

		macro_rules! test {
			($string:expr => $item:expr) => (
				assert_eq!(Control::DEC($item),
					parse($string).unwrap().1);
			);
		}

		#[test]
		fn decdhl() {
			test!(b"\x1B#3" =>
				DEC::Double(DEC::Half::Top));

			test!(b"\x1B#4" =>
				DEC::Double(DEC::Half::Bottom));
		}

		#[test]
		fn decswl() {
			test!(b"\x1B#5" =>
				DEC::SingleWidth);
		}

		#[test]
		fn decdwl() {
			test!(b"\x1B#6" =>
				DEC::DoubleWidth);
		}

		#[test]
		fn decaln() {
			test!(b"\x1B#8" =>
				DEC::AlignmentTest);
		}

		#[test]
		fn s7c1t() {
			test!(b"\x1B F" =>
				DEC::SevenBits);
		}

		#[test]
		fn s8c1t() {
			test!(b"\x1B G" =>
				DEC::EightBits);
		}

		#[test]
		fn scodfk() {
			test!(b"\x1BQ0'hue'" =>
				DEC::DefineFunctionKey(1, "hue"));

			test!(b"\x1BQaahuea" =>
				DEC::DefineFunctionKey(50, "hue"));
		}

		#[test]
		fn scs() {
			test!(b"\x1B(A" =>
				DEC::SelectCharset(0, DEC::Charset::ISO(DEC::charset::ISO::Latin1)));

			test!(b"\x1B)A" =>
				DEC::SelectCharset(1, DEC::Charset::ISO(DEC::charset::ISO::Latin1)));

			test!(b"\x1B*A" =>
				DEC::SelectCharset(2, DEC::Charset::ISO(DEC::charset::ISO::Latin1)));

			test!(b"\x1B+A" =>
				DEC::SelectCharset(3, DEC::Charset::ISO(DEC::charset::ISO::Latin1)));

			test!(b"\x1B-A" =>
				DEC::SelectCharset(1, DEC::Charset::ISO(DEC::charset::ISO::Latin1)));

			test!(b"\x1B.A" =>
				DEC::SelectCharset(2, DEC::Charset::ISO(DEC::charset::ISO::Latin1)));

			test!(b"\x1B/A" =>
				DEC::SelectCharset(3, DEC::Charset::ISO(DEC::charset::ISO::Latin1)));
		}

		#[test]
		fn ris() {
			test!(b"\x1Bc" =>
				DEC::ResetInitial);
		}

		#[test]
		fn decbi() {
			test!(b"\x1B6" =>
				DEC::BackIndex);
		}

		#[test]
		fn decsc() {
			test!(b"\x1B7" =>
				DEC::SaveCursor);
		}

		#[test]
		fn decrc() {
			test!(b"\x1B8" =>
				DEC::RestoreCursor);
		}

		#[test]
		fn decfi() {
			test!(b"\x1B9" =>
				DEC::ForwardIndex);
		}

		#[test]
		fn deckpam() {
			test!(b"\x1B=" =>
				DEC::ApplicationKeypad(true));
		}

		#[test]
		fn deckpnm() {
			test!(b"\x1B>" =>
				DEC::ApplicationKeypad(false));
		}

		#[test]
		fn decstbm() {
			test!(b"\x1B[r" =>
				DEC::ScrollRegion { top: 0, bottom: None });

			test!(b"\x1B[0;0r" =>
				DEC::ScrollRegion { top: 0, bottom: None });

			test!(b"\x1B[21;23r" =>
				DEC::ScrollRegion { top: 20, bottom: Some(22) });
		}

		#[test]
		fn decscusr() {
			test!(b"\x1B[ q" =>
				DEC::CursorStyle(0));

			test!(b"\x1B[3 q" =>
				DEC::CursorStyle(3));
		}

		#[test]
		fn decstr() {
			test!(b"\x1B[!p" =>
				DEC::SoftReset);
		}

		#[test]
		fn unicode() {
			test!(b"\x1B%G" =>
				DEC::Unicode(true));

			test!(b"\x1B%@" =>
				DEC::Unicode(false));
		}
	}

	mod format {
		use {Control, DEC, format, parse};

		macro_rules! test {
			($code:expr) => (
				let item = Control::DEC($code);

				assert_eq!(item, parse(&format(&item, true)).unwrap().1);
				assert_eq!(item, parse(&format(&item, false)).unwrap().1);
			);
		}

		#[test]
		fn decdhl() {
			test!(DEC::Double(DEC::Half::Top));
			test!(DEC::Double(DEC::Half::Bottom));
		}

		#[test]
		fn decswl() {
			test!(DEC::SingleWidth);
		}

		#[test]
		fn decdwl() {
			test!(DEC::DoubleWidth);
		}

		#[test]
		fn decaln() {
			test!(DEC::AlignmentTest);
		}

		#[test]
		fn s7c1t() {
			test!(DEC::SevenBits);
		}

		#[test]
		fn s8c1t() {
			test!(DEC::EightBits);
		}

		#[test]
		fn scodfk() {
			test!(DEC::DefineFunctionKey(1, "hue"));
			test!(DEC::DefineFunctionKey(50, "hue"));
		}

		#[test]
		fn scs() {
			test!(DEC::SelectCharset(0, DEC::Charset::ISO(DEC::charset::ISO::Latin1)));
			test!(DEC::SelectCharset(1, DEC::Charset::ISO(DEC::charset::ISO::Latin1)));
			test!(DEC::SelectCharset(2, DEC::Charset::ISO(DEC::charset::ISO::Latin1)));
			test!(DEC::SelectCharset(3, DEC::Charset::ISO(DEC::charset::ISO::Latin1)));
			test!(DEC::SelectCharset(1, DEC::Charset::ISO(DEC::charset::ISO::Latin1)));
			test!(DEC::SelectCharset(2, DEC::Charset::ISO(DEC::charset::ISO::Latin1)));
			test!(DEC::SelectCharset(3, DEC::Charset::ISO(DEC::charset::ISO::Latin1)));
		}

		#[test]
		fn ris() {
			test!(DEC::ResetInitial);
		}

		#[test]
		fn decbi() {
			test!(DEC::BackIndex);
		}

		#[test]
		fn decsc() {
			test!(DEC::SaveCursor);
		}

		#[test]
		fn decrc() {
			test!(DEC::RestoreCursor);
		}

		#[test]
		fn decfi() {
			test!(DEC::ForwardIndex);
		}

		#[test]
		fn deckpam() {
			test!(DEC::ApplicationKeypad(true));
		}

		#[test]
		fn deckpnm() {
			test!(DEC::ApplicationKeypad(false));
		}

		#[test]
		fn decstbm() {
			test!(DEC::ScrollRegion { top: 0, bottom: None });
			test!(DEC::ScrollRegion { top: 20, bottom: Some(22) });
		}

		#[test]
		fn unicode() {
			test!(DEC::Unicode(true));
			test!(DEC::Unicode(false));
		}

		#[test]
		fn decscusr() {
			test!(DEC::CursorStyle(0));
			test!(DEC::CursorStyle(3));
		}

		#[test]
		fn decstr() {
			test!(DEC::SoftReset);
		}
	}
}
