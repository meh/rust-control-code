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

use nom::{self, IResult, Needed};
use bit_vec::BitVec;
use CSI::parameters;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Header {
	pub aspect:     (u8, u8),
	pub background: bool,
	pub grid:       Option<u32>,
}

named!(pub header<Header>,
	do_parse!(
		args: parameters >>
		char!('q') >>

		(Header {
			aspect: match arg!(args[0] => 0) {
				2         => (5, 1),
				3 | 4     => (3, 1),
				5 | 6     => (2, 1),
				7 | 8 | 9 => (1, 1),
				0 | 1 | _ => (2, 1),
			},

			background: match arg!(args[1] => 1) {
				1     => false,
				2 | _ => true,
			},

			grid: arg!(args[2])
		})));

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Sixel {
	Value(BitVec),
	Repeat(u32, BitVec),

	Raster {
		aspect: (u32, u32),
		size:   (u32, u32),
	},

	Color(u32),
	Define(u32, Register),
	CarriageReturn,
	LineFeed,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Register {
	Rgb(u8, u8, u8),
	Hsl(u8, u8, u8),
}

#[inline]
pub fn parse(i: &[u8]) -> IResult<&[u8], Sixel> {
	if let IResult::Done(rest, value) = value(i) {
		IResult::Done(rest, Sixel::Value(value))
	}
	else {
		inner(i)
	}
}

named!(inner<Sixel>,
	alt!(repeat | raster | color | cr | lf));

fn value(i: &[u8]) -> IResult<&[u8], BitVec> {
	const TABLE: [u8; 256] = [
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1,
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
	];

	if i.is_empty() {
		return IResult::Incomplete(Needed::Unknown);
	}

	if TABLE[i[0] as usize] == 1 {
		let mut value = BitVec::from_bytes(&[(i[0] - 0x3F) << 2]);
		value.truncate(6);

		IResult::Done(&i[1..], value)
	}
	else {
		IResult::Error(nom::ErrorKind::Custom(0))
	}
}

named!(repeat<Sixel>,
	do_parse!(
		char!('!') >>
		args:  parameters >>
		value: value >>

		(Sixel::Repeat(arg!(args[0] => 1), value))));

named!(raster<Sixel>,
	do_parse!(
		char!('"') >>
		args: parameters >>

		(Sixel::Raster {
			aspect: (arg!(args[0] => 0), arg!(args[1] => 0)),
			size:   (arg!(args[2] => 0), arg!(args[3] => 0)),
		})));

named!(color<Sixel>,
	do_parse!(
		char!('#') >>
		args: parameters >>
		
		(if args.len() == 1 {
			Sixel::Color(arg!(args[0] => 0))
		}
		else {
			Sixel::Define(arg!(args[0] => 0), match arg!(args[1] => 0) {
				1     => Register::Hsl(arg!(args[2] => 0) as u8, arg!(args[3] => 0) as u8, arg!(args[4] => 0) as u8),
				2 | _ => Register::Rgb(arg!(args[2] => 0) as u8, arg!(args[3] => 0) as u8, arg!(args[4] => 0) as u8),
			})
		})));

named!(cr<Sixel>,
	value!(Sixel::CarriageReturn, tag!("$")));

named!(lf<Sixel>,
	value!(Sixel::LineFeed, tag!("-")));

pub mod shim {
	pub use super::Sixel as T;
	pub use super::Sixel::*;
	pub use super::Header;
	pub use super::{parse, header};
}

#[cfg(test)]
mod test {
	mod parse {
		use bit_vec::BitVec;
		use DEC::SIXEL::{self, parse, header};

		macro_rules! test {
			($string:expr => $item:expr) => (
				assert_eq!($item,
					parse($string).unwrap().1);
			);

			($ident:ident $string:expr => $item:expr) => (
				assert_eq!($item,
					$ident($string).unwrap().1);
			);

		}

		#[test]
		fn start() {
			test!(header b"q" =>
				SIXEL::Header { aspect: (2, 1), background: false, grid: None });

			test!(header b"0q" =>
				SIXEL::Header { aspect: (2, 1), background: false, grid: None });

			test!(header b"1q" =>
				SIXEL::Header { aspect: (2, 1), background: false, grid: None });

			test!(header b"2q" =>
				SIXEL::Header { aspect: (5, 1), background: false, grid: None });

			test!(header b"3q" =>
				SIXEL::Header { aspect: (3, 1), background: false, grid: None });

			test!(header b"4q" =>
				SIXEL::Header { aspect: (3, 1), background: false, grid: None });

			test!(header b"5q" =>
				SIXEL::Header { aspect: (2, 1), background: false, grid: None });

			test!(header b"6q" =>
				SIXEL::Header { aspect: (2, 1), background: false, grid: None });

			test!(header b"7q" =>
				SIXEL::Header { aspect: (1, 1), background: false, grid: None });

			test!(header b"8q" =>
				SIXEL::Header { aspect: (1, 1), background: false, grid: None });

			test!(header b"9q" =>
				SIXEL::Header { aspect: (1, 1), background: false, grid: None });

			test!(header b";2q" =>
				SIXEL::Header { aspect: (2, 1), background: true, grid: None });

			test!(header b";;100q" =>
				SIXEL::Header { aspect: (2, 1), background: false, grid: Some(100) });
		}

		#[test]
		fn value() {
			test!(b"?" =>
				SIXEL::Value(BitVec::from_elem(6, false)));

			test!(b"~" =>
				SIXEL::Value(BitVec::from_elem(6, true)));
		}
	}
}
