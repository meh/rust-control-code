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
use nom::{self, IResult, Needed, digit};
use {Format, CSI};
use util::number;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct Map(pub u8);

impl Map {
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.0 == 0
	}

	#[inline]
	pub fn get(&self, index: u8) -> bool {
		self.0 >> index & 1 == 1
	}

	#[inline]
	pub fn set(&mut self, index: u8, value: bool) {
		if value {
			self.0 |= 1 << index;
		}
		else {
			self.0 &= !(1 << index);
		}
	}
}

impl Format for Map {
	#[inline]
	fn fmt<W: Write>(&self, mut f: W) -> io::Result<()> {
		f.write_all(&[self.0 + 0x3F])
	}
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Header {
	pub aspect:     (u32, u32),
	pub background: bool,
	pub grid:       Option<u32>,
}

named!(pub header<Header>,
	do_parse!(
		args: call!(CSI::parameters) >>
		char!('q') >>

		(Header {
			aspect: match arg!(args[0] => 0) {
				1         => (2, 1),
				2         => (5, 1),
				3 | 4     => (3, 1),
				5 | 6     => (2, 1),
				7 | 8 | 9 => (1, 1),
				_         => (2, 1),
			},

			background: match arg!(args[1] => 1) {
				1     => false,
				2 | _ => true,
			},

			grid: arg!(args[2])
		})));

impl Format for Header {
	fn fmt<W: Write>(&self, mut f: W) -> io::Result<()> {
		if self.aspect != (2, 1) {
			try!(f.write_all(&[match self.aspect {
				(5, 1) => b'2',
				(3, 1) => b'3',
				(1, 1) => b'9',
				_      => b'0',
			}]));
		}

		if !self.background {
			try!(f.write_all(b";1"));
		}

		if let Some(grid) = self.grid {
			if self.background {
				try!(f.write_all(b";"));
			}

			try!(write!(f, "{}", grid));
		}

		f.write_all(b"q")
	}
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Sixel {
	Value(Map),
	Repeat(u32, Map),

	Raster {
		aspect: (u32, u32),
		size:   (u32, u32),
	},

	Enable(u32),
	Define(u32, Color),
	CarriageReturn,
	LineFeed,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Color {
	Hsl(u16, u8, u8),
	Rgb(u8, u8, u8),
	Rgba(u8, u8, u8, u8),
}

impl Format for Sixel {
	fn fmt<W: Write>(&self, mut f: W) -> io::Result<()> {
		match *self {
			Sixel::Value(value) => {
				try!(value.fmt(f.by_ref()));
			}

			Sixel::Repeat(times, value) => {
				try!(write!(f, "!{}", times));
				try!(value.fmt(f.by_ref()));
			}

			Sixel::Raster { aspect, size } => {
				try!(write!(f, "\"{};{};{};{}", aspect.0, aspect.1, size.0, size.1));
			}

			Sixel::Enable(id) => {
				try!(write!(f, "#{}", id));
			}

			Sixel::Define(id, color) => {
				try!(write!(f, "#{};", id));

				match color {
					Color::Hsl(h, s, l) => {
						try!(write!(f, "1;{};{};{}", h, l, s));
					}

					Color::Rgb(r, g, b) => {
						try!(write!(f, "2;{};{};{}",
							(r as f32 / 255.0 * 100.0) as u8,
							(g as f32 / 255.0 * 100.0) as u8,
							(b as f32 / 255.0 * 100.0) as u8));
					}

					Color::Rgba(r, g, b, a) => {
						try!(write!(f, "3;{};{};{};{}", r, g, b, a));
					}
				}
			}

			Sixel::CarriageReturn => {
				try!(f.write_all(b"$"));
			}

			Sixel::LineFeed => {
				try!(f.write_all(b"-"));
			}
		}

		Ok(())
	}
}

pub fn parse(i: &[u8]) -> IResult<&[u8], Sixel> {
	if let IResult::Done(rest, value) = value(i) {
		IResult::Done(rest, Sixel::Value(value))
	}
	else {
		inner(i)
	}
}

named!(inner<Sixel>,
	alt!(repeat | color | cr | lf | raster));

fn value(i: &[u8]) -> IResult<&[u8], Map> {
	const TABLE: [u8; 256] = [
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
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
		return IResult::Incomplete(Needed::Size(1));
	}

	if TABLE[i[0] as usize] == 1 {
		IResult::Done(&i[1..], Map(i[0] - 0x3F))
	}
	else {
		IResult::Error(nom::ErrorKind::Custom(0))
	}
}

named!(repeat<Sixel>,
	do_parse!(
		char!('!') >>
		count: digit >>
		value: value >>

		(Sixel::Repeat(number(count), value))));

named!(raster<Sixel>,
	do_parse!(
		char!('"') >>
		args: call!(CSI::parameters) >>

		(Sixel::Raster {
			aspect: (arg!(args[0] => 0), arg!(args[1] => 0)),
			size:   (arg!(args[2] => 0), arg!(args[3] => 0)),
		})));

named!(color<Sixel>,
	do_parse!(
		char!('#') >>
		id: digit >>

		color: opt!(switch!(take!(3),
			b";1;" => do_parse!(
				h: digit >>
				char!(';') >>
				l: digit >>
				char!(';') >>
				s: digit >>

				(Color::Hsl(number(h) as u16, number(s) as u8, number(l) as u8))) |

			b";2;" => do_parse!(
				r: digit >>
				char!(';') >>
				g: digit >>
				char!(';') >>
				b: digit >>

				(Color::Rgb(
					(number(r) as f32 / 100.0 * 255.0) as u8,
					(number(g) as f32 / 100.0 * 255.0) as u8,
					(number(b) as f32 / 100.0 * 255.0) as u8))) |

			b";3;" => do_parse!(
				r: digit >>
				char!(';') >>
				g: digit >>
				char!(';') >>
				b: digit >>
				char!(';') >>
				a: digit >>

				(Color::Rgba(number(r) as u8, number(g) as u8, number(b) as u8, number(a) as u8))))) >>

		(if let Some(color) = color {
			Sixel::Define(number(id), color)
		}
		else {
			Sixel::Enable(number(id))
		})));
	
named!(cr<Sixel>,
	value!(Sixel::CarriageReturn, char!('$')));

named!(lf<Sixel>,
	value!(Sixel::LineFeed, char!('-')));

pub mod shim {
	pub use super::Sixel as T;
	pub use super::Sixel::*;
	pub use super::{Header, Map, Color};
	pub use super::{parse, header};
}

#[cfg(test)]
mod test {
	mod parse {
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
				SIXEL::Value(SIXEL::Map(0b000000)));

			test!(b"~" =>
				SIXEL::Value(SIXEL::Map(0b111111)));
		}

		#[test]
		fn map() {
			let mut map = SIXEL::Map::default();

			map.set(0, true);
			assert_eq!(SIXEL::Map(1), map);

			map.set(0, false);
			assert_eq!(SIXEL::Map(0), map);

			map.set(1, true);
			assert_eq!(SIXEL::Map(2), map);
		}
	}

	mod format {
		use DEC::SIXEL::{self, parse};
		use format;

		macro_rules! test {
			($code:expr) => (
				let item = $code;
				assert_eq!(item, parse(&format(&item)).unwrap().1);
			);
		}

		#[test]
		fn repeat() {
			test!(SIXEL::Repeat(4, SIXEL::Map::default()));
		}
	}
}
