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
use {Format};

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum C1<'a> {
	PaddingCharacter,
	HighOctetPreset,
	BreakPermittedHere,
	NoBreakHere,
	Index,
	NextLine,
	StartSelectedArea,
	EndSelectedArea,
	HorizontalTabulationSet,
	HorizontalTabulationWithJustification,
	VerticalTabulationSet,
	PartialLineDown,
	PartialLineUp,
	ReverseIndex,
	SingleShiftTwo,
	SingleShiftThree,
	DeviceControlString(&'a str),
	PrivateUseOne,
	PrivateUseTwo,
	SetTransmitState,
	CancelCharacter,
	MessageWaiting,
	StartProtectedArea,
	EndProtectedArea,
	String(&'a str),
	// TODO: this should contain the value.
	SingleGraphicCharacter,
	SingleCharacter(&'a str),
	ControlSequence(::CSI::T),
	OperatingSystemCommand(&'a str),
	PrivacyMessage(&'a str),
	ApplicationProgramCommand(&'a str),
}

use self::C1::*;

impl<'a> Format for C1<'a> {
	fn fmt<W: Write>(&self, mut f: W, wide: bool) -> io::Result<()> {
		macro_rules! write {
			($code:expr) => (
				if wide {
					try!(f.write_all(&[0x1B, $code - 0x40]));
				}
				else {
					try!(f.write_all(&[$code]));
				}
			);
		}

		macro_rules! string {
			($string:expr) => (
				try!(f.write_all($string.as_bytes()));

				if wide {
					try!(f.write_all(&[0x1B, 0x9C - 0x40]));
				}
				else {
					try!(f.write_all(&[0x9C]));
				}
			)
		}

		match *self {
			PaddingCharacter =>
				write!(0x80),

			HighOctetPreset =>
				write!(0x81),

			BreakPermittedHere =>
				write!(0x82),

			NoBreakHere =>
				write!(0x83),

			Index =>
				write!(0x84),

			NextLine =>
				write!(0x85),

			StartSelectedArea =>
				write!(0x86),

			EndSelectedArea =>
				write!(0x87),

			HorizontalTabulationSet =>
				write!(0x88),

			HorizontalTabulationWithJustification =>
				write!(0x89),

			VerticalTabulationSet =>
				write!(0x8A),

			PartialLineDown =>
				write!(0x8B),

			PartialLineUp =>
				write!(0x8C),

			ReverseIndex =>
				write!(0x8D),

			SingleShiftTwo =>
				write!(0x8E),

			SingleShiftThree =>
				write!(0x8F),

			DeviceControlString(string) => {
				write!(0x90);
				string!(string);
			}

			PrivateUseOne =>
				write!(0x91),

			PrivateUseTwo =>
				write!(0x92),

			SetTransmitState =>
				write!(0x93),

			CancelCharacter =>
				write!(0x94),

			MessageWaiting =>
				write!(0x95),

			StartProtectedArea =>
				write!(0x96),

			EndProtectedArea =>
				write!(0x97),

			String(string) => {
				write!(0x98);
				string!(string);
			}

			SingleGraphicCharacter =>
				write!(0x99),

			SingleCharacter(string) => {
				write!(0x9A);
				string!(string);
			}

			ControlSequence(ref value) => {
				try!(value.fmt(f, wide));
			}

			OperatingSystemCommand(string) => {
				write!(0x9D);
				string!(string);
			}

			PrivacyMessage(string) => {
				write!(0x9E);
				string!(string);
			}

			ApplicationProgramCommand(string) => {
				write!(0x9F);
				string!(string);
			}
		}

		Ok(())
	}
}

named!(pub parse<C1>,
	alt!(one | two));

named!(one<C1>,
	switch!(take!(1),
		b"\x9B" => call!(CSI) |

		b"\x80" => call!(PAD)  |
		b"\x81" => call!(HOP)  |
		b"\x82" => call!(BPH)  |
		b"\x83" => call!(NBH)  |
		b"\x84" => call!(IND)  |
		b"\x85" => call!(NEL)  |
		b"\x86" => call!(SSA)  |
		b"\x87" => call!(ESA)  |
		b"\x88" => call!(HTS)  |
		b"\x89" => call!(HTJ)  |
		b"\x8A" => call!(VTS)  |
		b"\x8B" => call!(PLD)  |
		b"\x8C" => call!(PLU)  |
		b"\x8D" => call!(RI)   |
		b"\x8E" => call!(SS2)  |
		b"\x8F" => call!(SS3)  |
		b"\x90" => call!(DCS)  |
		b"\x91" => call!(PU1)  |
		b"\x92" => call!(PU2)  |
		b"\x93" => call!(STS)  |
		b"\x94" => call!(CCH)  |
		b"\x95" => call!(MW)   |
		b"\x96" => call!(SPA)  |
		b"\x97" => call!(EPA)  |
		b"\x98" => call!(SOS)  |
		b"\x99" => call!(SGCI) |
		b"\x9A" => call!(SCI)  |
		b"\x9D" => call!(OSC)  |
		b"\x9E" => call!(PM)   |
		b"\x9F" => call!(APC)));

named!(two<C1>,
	chain!(tag!(b"\x1B") ~
		res: switch!(take!(1),
			b"\x5B" => call!(CSI) |

			b"\x40" => call!(PAD)  |
			b"\x41" => call!(HOP)  |
			b"\x42" => call!(BPH)  |
			b"\x43" => call!(NBH)  |
			b"\x44" => call!(IND)  |
			b"\x45" => call!(NEL)  |
			b"\x46" => call!(SSA)  |
			b"\x47" => call!(ESA)  |
			b"\x48" => call!(HTS)  |
			b"\x49" => call!(HTJ)  |
			b"\x4A" => call!(VTS)  |
			b"\x4B" => call!(PLD)  |
			b"\x4C" => call!(PLU)  |
			b"\x4D" => call!(RI)   |
			b"\x4E" => call!(SS2)  |
			b"\x4F" => call!(SS3)  |
			b"\x50" => call!(DCS)  |
			b"\x51" => call!(PU1)  |
			b"\x52" => call!(PU2)  |
			b"\x53" => call!(STS)  |
			b"\x54" => call!(CCH)  |
			b"\x55" => call!(MW)   |
			b"\x56" => call!(SPA)  |
			b"\x57" => call!(EPA)  |
			b"\x58" => call!(SOS)  |
			b"\x59" => call!(SGCI) |
			b"\x5A" => call!(SCI)  |
			b"\x5D" => call!(OSC)  |
			b"\x5E" => call!(PM)   |
			b"\x5F" => call!(APC)),

	|| res));

named!(PAD<C1>,
	value!(PaddingCharacter));

named!(HOP<C1>,
	value!(HighOctetPreset));

named!(BPH<C1>,
	value!(BreakPermittedHere));

named!(NBH<C1>,
	value!(NoBreakHere));

named!(IND<C1>,
	value!(Index));

named!(NEL<C1>,
	value!(NextLine));

named!(SSA<C1>,
	value!(StartSelectedArea));

named!(ESA<C1>,
	value!(EndSelectedArea));

named!(HTS<C1>,
	value!(HorizontalTabulationSet));

named!(HTJ<C1>,
	value!(HorizontalTabulationWithJustification));

named!(VTS<C1>,
	value!(VerticalTabulationSet));

named!(PLD<C1>,
	value!(PartialLineDown));

named!(PLU<C1>,
	value!(PartialLineUp));

named!(RI<C1>,
	value!(ReverseIndex));

named!(SS2<C1>,
	value!(SingleShiftTwo));

named!(SS3<C1>,
	value!(SingleShiftThree));

named!(DCS<C1>,
	map!(string, |s| DeviceControlString(s)));

named!(PU1<C1>,
	value!(PrivateUseOne));

named!(PU2<C1>,
	value!(PrivateUseTwo));

named!(STS<C1>,
	value!(SetTransmitState));

named!(CCH<C1>,
	value!(CancelCharacter));

named!(MW<C1>,
	value!(MessageWaiting));

named!(SPA<C1>,
	value!(StartProtectedArea));

named!(EPA<C1>,
	value!(EndProtectedArea));

named!(SOS<C1>,
	map!(string, |s| String(s)));

named!(SGCI<C1>,
	value!(SingleGraphicCharacter));

named!(SCI<C1>,
	map!(string, |s| SingleCharacter(s)));

named!(CSI<C1>,
	map!(call!(::CSI::parse), |res| ControlSequence(res)));

named!(ST,
	alt!(tag!(b"\x9C") | tag!(b"\x1B\x5C")));

named!(OSC<C1>,
	map!(string, |s| OperatingSystemCommand(s)));

named!(PM<C1>,
	map!(string, |s| PrivacyMessage(s)));

named!(APC<C1>,
	map!(string, |s| ApplicationProgramCommand(s)));

named!(string<&str>,
	map!(terminated!(take_while!(is_string), ST),
		|s| unsafe { str::from_utf8_unchecked(s) }));

fn is_string(b: u8) -> bool {
	(b >= 0x08 && b <= 0x0D) || (b >= 0x20 && b <= 0x7E)
}

pub mod shim {
	pub use super::C1 as T;
	pub use super::C1::*;
	pub use super::parse;
}

#[cfg(test)]
mod test {
	mod parse {
		use {Control, C1, parse};

		macro_rules! test {
			($string:expr => $item:expr) => (
				assert_eq!(Control::C1($item),
					parse($string).unwrap().1);
			);
		}

		#[test]
		fn pad() {
			test!(b"\x80" =>
				C1::PaddingCharacter);

			test!(b"\x1B\x40" =>
				C1::PaddingCharacter);
		}

		#[test]
		fn hop() {
			test!(b"\x81" =>
				C1::HighOctetPreset);

			test!(b"\x1B\x41" =>
				C1::HighOctetPreset);
		}

		#[test]
		fn bph() {
			test!(b"\x82" =>
				C1::BreakPermittedHere);

			test!(b"\x1B\x42" =>
				C1::BreakPermittedHere);
		}

		#[test]
		fn nbh() {
			test!(b"\x83" =>
				C1::NoBreakHere);

			test!(b"\x1B\x43" =>
				C1::NoBreakHere);
		}

		#[test]
		fn ind() {
			test!(b"\x84" =>
				C1::Index);

			test!(b"\x1B\x44" =>
				C1::Index);
		}

		#[test]
		fn nel() {
			test!(b"\x85" =>
				C1::NextLine);

			test!(b"\x1B\x45" =>
				C1::NextLine);
		}

		#[test]
		fn ssa() {
			test!(b"\x86" =>
				C1::StartSelectedArea);

			test!(b"\x1B\x46" =>
				C1::StartSelectedArea);
		}

		#[test]
		fn esa() {
			test!(b"\x87" =>
				C1::EndSelectedArea);

			test!(b"\x1B\x47" =>
				C1::EndSelectedArea);
		}

		#[test]
		fn hts() {
			test!(b"\x88" =>
				C1::HorizontalTabulationSet);

			test!(b"\x1B\x48" =>
				C1::HorizontalTabulationSet);
		}

		#[test]
		fn htj() {
			test!(b"\x89" =>
				C1::HorizontalTabulationWithJustification);

			test!(b"\x1B\x49" =>
				C1::HorizontalTabulationWithJustification);
		}

		#[test]
		fn vts() {
			test!(b"\x8A" =>
				C1::VerticalTabulationSet);

			test!(b"\x1B\x4A" =>
				C1::VerticalTabulationSet);
		}

		#[test]
		fn pld() {
			test!(b"\x8B" =>
				C1::PartialLineDown);

			test!(b"\x1B\x4B" =>
				C1::PartialLineDown);
		}

		#[test]
		fn plu() {
			test!(b"\x8C" =>
				C1::PartialLineUp);

			test!(b"\x1B\x4C" =>
				C1::PartialLineUp);
		}

		#[test]
		fn ri() {
			test!(b"\x8D" =>
				C1::ReverseIndex);

			test!(b"\x1B\x4D" =>
				C1::ReverseIndex);
		}

		#[test]
		fn ss2() {
			test!(b"\x8E" =>
				C1::SingleShiftTwo);

			test!(b"\x1B\x4E" =>
				C1::SingleShiftTwo);
		}

		#[test]
		fn ss3() {
			test!(b"\x8F" =>
				C1::SingleShiftThree);

			test!(b"\x1B\x4F" =>
				C1::SingleShiftThree);
		}

		#[test]
		fn dcs() {
			test!(b"\x90foo\x9C" =>
				C1::DeviceControlString("foo"));

			test!(b"\x1B\x50foo\x1B\x5C" =>
				C1::DeviceControlString("foo"));
		}

		#[test]
		fn pu1() {
			test!(b"\x91" =>
				C1::PrivateUseOne);

			test!(b"\x1B\x51" =>
				C1::PrivateUseOne);
		}

		#[test]
		fn pu2() {
			test!(b"\x92" =>
				C1::PrivateUseTwo);

			test!(b"\x1B\x52" =>
				C1::PrivateUseTwo);
		}

		#[test]
		fn sts() {
			test!(b"\x93" =>
				C1::SetTransmitState);

			test!(b"\x1B\x53" =>
				C1::SetTransmitState);
		}

		#[test]
		fn cch() {
			test!(b"\x94" =>
				C1::CancelCharacter);

			test!(b"\x1B\x54" =>
				C1::CancelCharacter);
		}

		#[test]
		fn mw() {
			test!(b"\x95" =>
				C1::MessageWaiting);

			test!(b"\x1B\x55" =>
				C1::MessageWaiting);
		}

		#[test]
		fn spa() {
			test!(b"\x96" =>
				C1::StartProtectedArea);

			test!(b"\x1B\x56" =>
				C1::StartProtectedArea);
		}

		#[test]
		fn epa() {
			test!(b"\x97" =>
				C1::EndProtectedArea);

			test!(b"\x1B\x57" =>
				C1::EndProtectedArea);
		}

		#[test]
		fn sos() {
			test!(b"\x98foo\x9C" =>
				C1::String("foo"));

			test!(b"\x1B\x58foo\x1B\x5C" =>
				C1::String("foo"));
		}

		#[test]
		fn sgci() {
			test!(b"\x99" =>
				C1::SingleGraphicCharacter);

			test!(b"\x1B\x59" =>
				C1::SingleGraphicCharacter);
		}

		#[test]
		fn sci() {
			test!(b"\x9Afoo\x9C" =>
				C1::SingleCharacter("foo"));

			test!(b"\x1B\x5Afoo\x1B\x5C" =>
				C1::SingleCharacter("foo"));
		}

		#[test]
		fn osc() {
			test!(b"\x9Dfoo\x9C" =>
				C1::OperatingSystemCommand("foo"));

			test!(b"\x1B\x5Dfoo\x1B\x5C" =>
				C1::OperatingSystemCommand("foo"));
		}

		#[test]
		fn pn() {
			test!(b"\x9Efoo\x9C" =>
				C1::PrivacyMessage("foo"));

			test!(b"\x1B\x5Efoo\x1B\x5C" =>
				C1::PrivacyMessage("foo"));
		}

		#[test]
		fn apc() {
			test!(b"\x9Ffoo\x9C" =>
				C1::ApplicationProgramCommand("foo"));

			test!(b"\x1B\x5Ffoo\x1B\x5C" =>
				C1::ApplicationProgramCommand("foo"));
		}
	}

	mod format {
		use {Control, C1, format, parse};

		macro_rules! test {
			($code:expr) => (
				let item = Control::C1($code);

				assert_eq!(item, parse(&format(&item, true)).unwrap().1);
				assert_eq!(item, parse(&format(&item, false)).unwrap().1);
			);
		}

		#[test]
		fn pad() {
			test!(C1::PaddingCharacter);
		}

		#[test]
		fn hop() {
			test!(C1::HighOctetPreset);
		}

		#[test]
		fn bph() {
			test!(C1::BreakPermittedHere);
		}

		#[test]
		fn nbh() {
			test!(C1::NoBreakHere);
		}

		#[test]
		fn ind() {
			test!(C1::Index);
		}

		#[test]
		fn nel() {
			test!(C1::NextLine);
		}

		#[test]
		fn ssa() {
			test!(C1::StartSelectedArea);
		}

		#[test]
		fn esa() {
			test!(C1::EndSelectedArea);
		}

		#[test]
		fn hts() {
			test!(C1::HorizontalTabulationSet);
		}

		#[test]
		fn htj() {
			test!(C1::HorizontalTabulationWithJustification);
		}

		#[test]
		fn vts() {
			test!(C1::VerticalTabulationSet);
		}

		#[test]
		fn pld() {
			test!(C1::PartialLineDown);
		}

		#[test]
		fn plu() {
			test!(C1::PartialLineUp);
		}

		#[test]
		fn ri() {
			test!(C1::ReverseIndex);
		}

		#[test]
		fn ss2() {
			test!(C1::SingleShiftTwo);
		}

		#[test]
		fn ss3() {
			test!(C1::SingleShiftThree);
		}

		#[test]
		fn dcs() {
			test!(C1::DeviceControlString("foo"));
		}

		#[test]
		fn pu1() {
			test!(C1::PrivateUseOne);
		}

		#[test]
		fn pu2() {
			test!(C1::PrivateUseTwo);
		}

		#[test]
		fn sts() {
			test!(C1::SetTransmitState);
		}

		#[test]
		fn cch() {
			test!(C1::CancelCharacter);
		}

		#[test]
		fn mw() {
			test!(C1::MessageWaiting);
		}

		#[test]
		fn spa() {
			test!(C1::StartProtectedArea);
		}

		#[test]
		fn epa() {
			test!(C1::EndProtectedArea);
		}

		#[test]
		fn sos() {
			test!(C1::String("foo"));
		}

		#[test]
		fn sgci() {
			test!(C1::SingleGraphicCharacter);
		}

		#[test]
		fn sci() {
			test!(C1::SingleCharacter("foo"));
		}

		#[test]
		fn osc() {
			test!(C1::OperatingSystemCommand("foo"));
		}

		#[test]
		fn pn() {
			test!(C1::PrivacyMessage("foo"));
		}

		#[test]
		fn apc() {
			test!(C1::ApplicationProgramCommand("foo"));
		}
	}
}
