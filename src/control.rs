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

use std::str;
use std::io::{self, Write};
use nom::{self, IResult, Needed};

use {Format, C0, C1, DEC, CSI, SGR};

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Control<'a> {
	C0(C0::T),
	C1(C1::T<'a>),
	DEC(DEC::T<'a>),
	None(&'a str),
}

impl<'a> From<C0::T> for Control<'a> {
	fn from(value: C0::T) -> Control<'a> {
		Control::C0(value)
	}
}

impl<'a> From<C1::T<'a>> for Control<'a> {
	fn from(value: C1::T<'a>) -> Control<'a> {
		Control::C1(value)
	}
}

impl<'a> From<DEC::T<'a>> for Control<'a> {
	fn from(value: DEC::T<'a>) -> Control<'a> {
		Control::DEC(value)
	}
}

impl<'a> From<CSI::T> for Control<'a> {
	fn from(value: CSI::T) -> Control<'a> {
		Control::C1(C1::ControlSequence(value))
	}
}

impl<'a> From<SGR::T> for Control<'a> {
	fn from(value: SGR::T) -> Control<'a> {
		Control::C1(C1::ControlSequence(CSI::SelectGraphicalRendition(vec![value])))
	}
}

impl<'a> From<Vec<SGR::T>> for Control<'a> {
	fn from(value: Vec<SGR::T>) -> Control<'a> {
		Control::C1(C1::ControlSequence(CSI::SelectGraphicalRendition(value)))
	}
}

impl<'a> Format for Control<'a> {
	fn fmt<W: Write>(&self, mut f: W, wide: bool) -> io::Result<()> {
		match self {
			&Control::None(ref value) =>
				f.write_all(value.as_bytes()),

			&Control::C0(ref value) =>
				value.fmt(f, wide),

			&Control::C1(ref value) =>
				value.fmt(f, wide),

			&Control::DEC(ref value) =>
				value.fmt(f, wide),
		}
	}
}

named!(pub parse<Control>,
	alt!(control | string));

fn string(i: &[u8]) -> IResult<&[u8], Control> {
	const WIDTH: [u8; 256] = [
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x1F
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x3F
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x5F
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x7F
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x9F
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xBF
		0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
		2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // 0xDF
		3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // 0xEF
		4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xFF
	];

	let mut length = WIDTH[i[0] as usize] as usize;

	if i.len() < length {
		return IResult::Incomplete(Needed::Size(length - i.len()));
	}

	let mut rest = &i[length..];

	while !rest.is_empty() && control(rest).is_err() {
		let w = WIDTH[rest[0] as usize] as usize;

		if rest.len() < w {
			return IResult::Incomplete(Needed::Size(w - rest.len()));
		}

		length += w;
		rest    = &rest[w..];
	}

	if let Ok(string) = str::from_utf8(&i[..length]) {
		IResult::Done(&i[length..], Control::None(string))
	}
	else {
		IResult::Error(nom::Err::Code(nom::ErrorKind::Custom(9001)))
	}
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

			_ =>
				Control::C1(c)
		})
		|
		map!(C0::parse,  |c| Control::C0(c))));
