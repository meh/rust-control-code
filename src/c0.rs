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
use Format;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum C0 {
	Null,
	StartHeading,
	StartText,
	EndText,
	EndTransmission,
	Enquiry,
	Acknowledge,
	Bell,
	Backspace,
	HorizontalTabulation,
	LineFeed,
	VerticalTabulation,
	FormFeed,
	CarriageReturn,
	ShiftOut,
	ShiftIn,
	DataLinkEscape,
	DeviceControlOne,
	DeviceControlTwo,
	DeviceControlThree,
	DeviceControlFour,
	NegativeAcknowledge,
	SynchronousIdle,
	EndTransmissionBlock,
	Cancel,
	EndMedium,
	Substitute,
	Escape,
	FileSeparator,
	GroupSeparator,
	RecordSeparator,
	UnitSeparator,
}

use self::C0::*;

impl Format for C0 {
	fn fmt<W: Write>(&self, mut f: W, _wide: bool) -> io::Result<()> {
		macro_rules! write {
			($code:expr) => (
				f.write_all(&[$code])
			);
		}

		match *self {
			Null =>
				write!(0x00),

			StartHeading =>
				write!(0x01),

			StartText =>
				write!(0x02),

			EndText =>
				write!(0x03),

			EndTransmission =>
				write!(0x04),

			Enquiry =>
				write!(0x05),

			Acknowledge =>
				write!(0x06),

			Bell =>
				write!(0x07),

			Backspace =>
				write!(0x08),

			HorizontalTabulation =>
				write!(0x09),

			LineFeed =>
				write!(0x0A),

			VerticalTabulation =>
				write!(0x0B),

			FormFeed =>
				write!(0x0C),

			CarriageReturn =>
				write!(0x0D),

			ShiftOut =>
				write!(0x0E),

			ShiftIn =>
				write!(0x0F),

			DataLinkEscape =>
				write!(0x10),

			DeviceControlOne =>
				write!(0x11),

			DeviceControlTwo =>
				write!(0x12),

			DeviceControlThree =>
				write!(0x13),

			DeviceControlFour =>
				write!(0x14),

			NegativeAcknowledge =>
				write!(0x15),

			SynchronousIdle =>
				write!(0x16),

			EndTransmissionBlock =>
				write!(0x17),

			Cancel =>
				write!(0x18),

			EndMedium =>
				write!(0x19),

			Substitute =>
				write!(0x1A),

			Escape =>
				write!(0x1B),

			FileSeparator =>
				write!(0x1C),

			GroupSeparator =>
				write!(0x1D),

			RecordSeparator =>
				write!(0x1E),

			UnitSeparator =>
				write!(0x1F),
		}
	}
}

named!(pub parse<C0>,
	switch!(take!(1),
		b"\x08" => call!(BS)  |
		b"\x09" => call!(HT)  |
		b"\x0A" => call!(LF)  |
		b"\x0D" => call!(CR)  |
		b"\x1B" => call!(ESC) |

		b"\x00" => call!(NUL) |
		b"\x01" => call!(SOH) |
		b"\x02" => call!(STX) |
		b"\x03" => call!(ETX) |
		b"\x04" => call!(EOT) |
		b"\x05" => call!(ENQ) |
		b"\x06" => call!(ACK) |
		b"\x07" => call!(BEL) |
		b"\x0B" => call!(VT)  |
		b"\x0C" => call!(FF)  |
		b"\x0E" => call!(SS)  |
		b"\x0F" => call!(SI)  |
		b"\x10" => call!(DLE) |
		b"\x11" => call!(DC1) |
		b"\x12" => call!(DC2) |
		b"\x13" => call!(DC3) |
		b"\x14" => call!(DC4) |
		b"\x15" => call!(NAK) |
		b"\x16" => call!(SYN) |
		b"\x17" => call!(ETB) |
		b"\x18" => call!(CAN) |
		b"\x19" => call!(EM)  |
		b"\x1A" => call!(SUB) |
		b"\x1C" => call!(FS)  |
		b"\x1D" => call!(GS)  |
		b"\x1E" => call!(RS)  |
		b"\x1F" => call!(US)));

named!(NUL<C0>,
	value!(Null));

named!(SOH<C0>,
	value!(StartHeading));

named!(STX<C0>,
	value!(StartText));

named!(ETX<C0>,
	value!(EndText));

named!(EOT<C0>,
	value!(EndTransmission));

named!(ENQ<C0>,
	value!(Enquiry));

named!(ACK<C0>,
	value!(Acknowledge));

named!(BEL<C0>,
	value!(Bell));

named!(BS<C0>,
	value!(Backspace));

named!(HT<C0>,
	value!(HorizontalTabulation));

named!(LF<C0>,
	value!(LineFeed));

named!(VT<C0>,
	value!(VerticalTabulation));

named!(FF<C0>,
	value!(FormFeed));

named!(CR<C0>,
	value!(CarriageReturn));

named!(SS<C0>,
	value!(ShiftOut));

named!(SI<C0>,
	value!(ShiftIn));

named!(DLE<C0>,
	value!(DataLinkEscape));

named!(DC1<C0>,
	value!(DeviceControlOne));

named!(DC2<C0>,
	value!(DeviceControlTwo));

named!(DC3<C0>,
	value!(DeviceControlThree));

named!(DC4<C0>,
	value!(DeviceControlFour));

named!(NAK<C0>,
	value!(NegativeAcknowledge));

named!(SYN<C0>,
	value!(SynchronousIdle));

named!(ETB<C0>,
	value!(EndTransmissionBlock));

named!(CAN<C0>,
	value!(Cancel));

named!(EM<C0>,
	value!(EndMedium));

named!(SUB<C0>,
	value!(Substitute));

named!(ESC<C0>,
	value!(Escape));

named!(FS<C0>,
	value!(FileSeparator));

named!(GS<C0>,
	value!(GroupSeparator));

named!(RS<C0>,
	value!(RecordSeparator));

named!(US<C0>,
	value!(UnitSeparator));

pub mod shim {
	pub use super::C0 as T;
	pub use super::C0::*;
	pub use super::parse;
}

#[cfg(test)]
mod test {
	mod parse {
		use {Control, C0, parse};

		macro_rules! test {
			($id:expr => $attr:expr) => (
				assert_eq!(Control::C0($attr),
					parse(&[$id]).unwrap().1);
			);
		}

		#[test]
		fn nul() {
			test!(0x00 =>
				C0::Null);
		}

		#[test]
		fn soh() {
			test!(0x01 =>
				C0::StartHeading);
		}

		#[test]
		fn stx() {
			test!(0x02 =>
				C0::StartText);
		}

		#[test]
		fn etx() {
			test!(0x03 =>
				C0::EndText);
		}

		#[test]
		fn eot() {
			test!(0x04 =>
				C0::EndTransmission);
		}

		#[test]
		fn enq() {
			test!(0x05 =>
				C0::Enquiry);
		}

		#[test]
		fn ack() {
			test!(0x06 =>
				C0::Acknowledge);
		}

		#[test]
		fn bel() {
			test!(0x07 =>
				C0::Bell);
		}

		#[test]
		fn bs() {
			test!(0x08 =>
				C0::Backspace);
		}

		#[test]
		fn ht() {
			test!(0x09 =>
				C0::HorizontalTabulation);
		}

		#[test]
		fn lf() {
			test!(0x0A =>
				C0::LineFeed);
		}

		#[test]
		fn vf() {
			test!(0x0B =>
				C0::VerticalTabulation);
		}

		#[test]
		fn ff() {
			test!(0x0C =>
				C0::FormFeed);
		}

		#[test]
		fn cr() {
			test!(0x0D =>
				C0::CarriageReturn);
		}

		#[test]
		fn ss() {
			test!(0x0E =>
				C0::ShiftOut);
		}

		#[test]
		fn si() {
			test!(0x0F =>
				C0::ShiftIn);
		}

		#[test]
		fn dle() {
			test!(0x10 =>
				C0::DataLinkEscape);
		}

		#[test]
		fn dc1() {
			test!(0x11 =>
				C0::DeviceControlOne);
		}

		#[test]
		fn dc2() {
			test!(0x12 =>
				C0::DeviceControlTwo);
		}

		#[test]
		fn dc3() {
			test!(0x13 =>
				C0::DeviceControlThree);
		}

		#[test]
		fn dc4() {
			test!(0x14 =>
				C0::DeviceControlFour);
		}

		#[test]
		fn nak() {
			test!(0x15 =>
				C0::NegativeAcknowledge);
		}

		#[test]
		fn syn() {
			test!(0x16 =>
				C0::SynchronousIdle);
		}

		#[test]
		fn etb() {
			test!(0x17 =>
				C0::EndTransmissionBlock);
		}

		#[test]
		fn can() {
			test!(0x18 =>
				C0::Cancel);
		}

		#[test]
		fn em() {
			test!(0x19 =>
				C0::EndMedium);
		}

		#[test]
		fn sub() {
			test!(0x1A =>
				C0::Substitute);
		}

		#[test]
		fn fs() {
			test!(0x1C =>
				C0::FileSeparator);
		}

		#[test]
		fn gs() {
			test!(0x1D =>
				C0::GroupSeparator);
		}

		#[test]
		fn rs() {
			test!(0x1E =>
				C0::RecordSeparator);
		}

		#[test]
		fn us() {
			test!(0x1F =>
				C0::UnitSeparator);
		}
	}

	mod format {
		use {Control, C0, format, parse};

		macro_rules! test {
			($code:expr) => (
				let item = Control::C0($code);

				assert_eq!(item, parse(&format(&item, true)).unwrap().1);
				assert_eq!(item, parse(&format(&item, false)).unwrap().1);
			);
		}

		#[test]
		fn nul() {
			test!(C0::Null);
		}

		#[test]
		fn soh() {
			test!(C0::StartHeading);
		}

		#[test]
		fn stx() {
			test!(C0::StartText);
		}

		#[test]
		fn etx() {
			test!(C0::EndText);
		}

		#[test]
		fn eot() {
			test!(C0::EndTransmission);
		}

		#[test]
		fn enq() {
			test!(C0::Enquiry);
		}

		#[test]
		fn ack() {
			test!(C0::Acknowledge);
		}

		#[test]
		fn bel() {
			test!(C0::Bell);
		}

		#[test]
		fn bs() {
			test!(C0::Backspace);
		}

		#[test]
		fn ht() {
			test!(C0::HorizontalTabulation);
		}

		#[test]
		fn lf() {
			test!(C0::LineFeed);
		}

		#[test]
		fn vf() {
			test!(C0::VerticalTabulation);
		}

		#[test]
		fn ff() {
			test!(C0::FormFeed);
		}

		#[test]
		fn cr() {
			test!(C0::CarriageReturn);
		}

		#[test]
		fn ss() {
			test!(C0::ShiftOut);
		}

		#[test]
		fn si() {
			test!(C0::ShiftIn);
		}

		#[test]
		fn dle() {
			test!(C0::DataLinkEscape);
		}

		#[test]
		fn dc1() {
			test!(C0::DeviceControlOne);
		}

		#[test]
		fn dc2() {
			test!(C0::DeviceControlTwo);
		}

		#[test]
		fn dc3() {
			test!(C0::DeviceControlThree);
		}

		#[test]
		fn dc4() {
			test!(C0::DeviceControlFour);
		}

		#[test]
		fn nak() {
			test!(C0::NegativeAcknowledge);
		}

		#[test]
		fn syn() {
			test!(C0::SynchronousIdle);
		}

		#[test]
		fn etb() {
			test!(C0::EndTransmissionBlock);
		}

		#[test]
		fn can() {
			test!(C0::Cancel);
		}

		#[test]
		fn em() {
			test!(C0::EndMedium);
		}

		#[test]
		fn sub() {
			test!(C0::Substitute);
		}

		#[test]
		fn fs() {
			test!(C0::FileSeparator);
		}

		#[test]
		fn gs() {
			test!(C0::GroupSeparator);
		}

		#[test]
		fn rs() {
			test!(C0::RecordSeparator);
		}

		#[test]
		fn us() {
			test!(C0::UnitSeparator);
		}
	}
}
