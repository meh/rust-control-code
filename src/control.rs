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
use smallvec::SmallVec;
use nom::{self, IResult, Needed};

use {Format, C0, C1, DEC, CSI, SGR};

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Control<'a> {
	C0(C0::T),
	C1(C1::T),
	DEC(DEC::T<'a>),
}

impl<'a> From<C0::T> for Control<'a> {
	#[inline]
	fn from(value: C0::T) -> Control<'a> {
		Control::C0(value)
	}
}

impl<'a> From<C1::T> for Control<'a> {
	#[inline]
	fn from(value: C1::T) -> Control<'a> {
		Control::C1(value)
	}
}

impl<'a> From<DEC::T<'a>> for Control<'a> {
	#[inline]
	fn from(value: DEC::T<'a>) -> Control<'a> {
		Control::DEC(value)
	}
}

impl<'a> From<CSI::T> for Control<'a> {
	#[inline]
	fn from(value: CSI::T) -> Control<'a> {
		Control::C1(C1::ControlSequence(value))
	}
}

impl<'a> From<SGR::T> for Control<'a> {
	#[inline]
	fn from(value: SGR::T) -> Control<'a> {
		Control::C1(C1::ControlSequence(CSI::SelectGraphicalRendition(small_vec![value])))
	}
}

impl<'a> From<SmallVec<[SGR::T; CSI::SIZE]>> for Control<'a> {
	#[inline]
	fn from(value: SmallVec<[SGR::T; CSI::SIZE]>) -> Control<'a> {
		Control::C1(C1::ControlSequence(CSI::SelectGraphicalRendition(value)))
	}
}

impl<'a> Format for Control<'a> {
	#[inline]
	fn fmt<W: Write>(&self, f: W, wide: bool) -> io::Result<()> {
		match self {
			&Control::C0(ref value) =>
				value.fmt(f, wide),

			&Control::C1(ref value) =>
				value.fmt(f, wide),

			&Control::DEC(ref value) =>
				value.fmt(f, wide),
		}
	}
}

/// Parse a control code.
pub fn parse(i: &[u8]) -> IResult<&[u8], Control> {
	const TABLE: [u8; 256] = [
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	];

	if i.is_empty() {
		return IResult::Incomplete(Needed::Unknown);
	}

	if TABLE[i[0] as usize] == 0 {
		return IResult::Error(nom::ErrorKind::Custom(0));
	}

	control(i)
}

named!(control<Control>,
	alt!(
		map!(DEC::parse, |c| Control::DEC(c))
		|
		map!(C1::parse, |c| match c {
			C1::ControlSequence(CSI::Unknown(id, modifier, args)) => {
				if let Some(c) = DEC::CSI::normal(id, modifier, &args) {
					Control::DEC(c)
				}
				else {
					Control::C1(C1::ControlSequence(CSI::Unknown(id, modifier, args)))
				}
			}

			C1::ControlSequence(CSI::Private(id, modifier, args)) => {
				if let Some(c) = DEC::CSI::private(id, modifier, &args) {
					Control::DEC(c)
				}
				else {
					Control::C1(C1::ControlSequence(CSI::Private(id, modifier, args)))
				}
			}

			control =>
				Control::C1(control)
		})
		|
		map!(C0::parse,  |c| Control::C0(c))));
