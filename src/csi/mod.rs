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
use std::u32;
use nom::{self, ErrorKind};

use std::io::{self, Write};
use Format;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum CSI {
	CursorBackTabulation(u32),
	CursorHorizontalPosition(u32),
	CursorForwardTabulation(u32),
	CursorNextLine(u32),
	CursorPreviousLine(u32),
	CursorPositionReport { x: u32, y: u32 },
	CursorTabulationControl(Tabulation),
	CursorBack(u32),
	CursorDown(u32),
	CursorForward(u32),
	CursorPosition { x: u32, y: u32 },
	CursorUp(u32),
	CursorLineTabulation(u32),
	DeviceAttributes(u32),
	DefineAreaQualification(Qualification),
	DeleteCharacter(u32),
	DeleteLine(u32),
	DeviceStatusReport,
	DimensionTextArea(u32, u32),
	EraseArea(Erase),
	EraseCharacter(u32),
	EraseDisplay(Erase),
	EraseField(Erase),
	EraseLine(Erase),
	FunctionKey(u32),
	SelectFont(u32, u32),
	GraphicCharacterCombination(Combination),
	GraphicSizeModification { width: u32, height: u32 },
	InsertCharacter(u32),
	IdentifyDeviceControlString(Option<u32>),
	IdentifyGraphicSubrepertoire(Option<u32>),
	InsertLine(u32),
	Justify(Vec<Option<u32>>),
	MediaCopy(Copy),
	NextPage(u32),
	Presentation(Expansion),
	PageFormat(u32),
	PrecedingPage(u32),
	PagePosition(u32),
	PageBack(u32),
	PageForward(u32),
	ParallelText(Parallel),
	GraphicDisposition(Vec<Disposition>),
	RestoreCursor,
	Repeat(u32),
	Reset(Vec<Mode>),
	CharacterOrientation(u16),
	SaveCursor,
	CharacterSpacing(u32),
	ScrollDown(u32),
	Movement(Direction),
	SelectGraphicalRendition(Vec<::SGR::T>),
	ScrollLeft(u32),
	LineSpacing(u32),
	Set(Vec<Mode>),
	ScrollRight(u32),
	ReverseString(bool),
	SizeUnit(Unit),
	SpaceWidth(u32),
	ScrollUp(u32),
	TabulationClear(Tabulation),
	LinePosition(u32),

	Unknown(u8, Option<u8>, Vec<Option<u32>>),
	Private(u8, Option<u8>, Vec<Option<u32>>),
}

use self::CSI::*;

impl Format for CSI {
	fn fmt<W: Write>(&self, mut f: W, wide: bool) -> io::Result<()> {
		macro_rules! write {
			(entry $private:expr) => ({
				if wide {
					try!(f.write_all(b"\x1B\x5B"));
				}
				else {
					try!(f.write_all(b"\x9B"));
				}

				if $private {
					try!(f.write_all(b"?"));
				}
			});

			(parameters $args:expr) => ({
				let     iter = $args;
				let mut iter = iter.peekable();

				while let Some(value) = iter.next() {
					if let Some(value) = value.clone() {
						let value: u32 = value;
						try!(f.write_all(value.to_string().as_bytes()));
					}

					if iter.peek().is_some() {
						try!(f.write_all(&[b';']));
					}
				}
			});

			(identifier $id:expr, $modifier:expr) => ({
				let id       = $id as u8;
				let modifier = $modifier.map(|c| c as u8);

				if let Some(modifier) = modifier {
					try!(f.write_all(&[modifier]));
				}

				f.write_all(&[id])
			});

			($id:expr, [$($values:expr),*]) => ({
				let params = [$(Some(Into::<u32>::into($values))),*];
				write!($id, params.iter())
			});

			($id:expr, ![$($values:expr),*]) => ({
				let params = [$($values),*];
				write!($id, params.iter())
			});

			($id:expr, $iter:expr) => ({
				write!(entry false);
				write!(parameters $iter);

				f.write_all($id.as_bytes())
			});

			($id:expr) => ({
				write!(entry false);
				f.write_all($id.as_bytes())
			});
		}

		match *self {
			CursorBackTabulation(n) =>
				write!("Z", [n]),

			CursorHorizontalPosition(n) =>
				write!("G", [n + 1]),

			CursorForwardTabulation(n) =>
				write!("I", [n]),

			CursorNextLine(n) =>
				write!("E", [n]),

			CursorPreviousLine(n) =>
				write!("F", [n]),

			CursorPositionReport { x, y } =>
				write!("R", [y + 1, x + 1]),

			CursorTabulationControl(value) =>
				write!("W", [value]),

			CursorBack(n) =>
				write!("D", [n]),

			CursorDown(n) =>
				write!("B", [n]),

			CursorForward(n) =>
				write!("C", [n]),

			CursorPosition { x, y } =>
				write!("H", [y + 1, x + 1]),

			CursorUp(n) =>
				write!("A", [n]),

			CursorLineTabulation(n) =>
				write!("Y", [n]),

			DeviceAttributes(n) =>
				write!("c", [n]),

			DefineAreaQualification(value) =>
				write!("o", [value]),

			DeleteCharacter(n) =>
				write!("P", [n]),

			DeleteLine(n) =>
				write!("M", [n]),

			DeviceStatusReport =>
				write!("n", [6u32]),

			DimensionTextArea(w, h) =>
				write!(" T", [w, h]),

			EraseArea(value) =>
				write!("O", [value]),

			EraseCharacter(n) =>
				write!("X", [n]),

			EraseField(value) =>
				write!("N", [value]),

			EraseDisplay(value) =>
				write!("J", [value]),

			EraseLine(value) =>
				write!("K", [value]),

			FunctionKey(n) =>
				write!(" W", [n]),

			SelectFont(a, b) =>
				write!(" D", [a, b]),

			GraphicCharacterCombination(value) =>
				write!(" _", [value]),

			GraphicSizeModification { width, height } =>
				write!(" B", [height, width]),

			InsertCharacter(n) =>
				write!("@", [n]),

			IdentifyDeviceControlString(n) =>
				write!(" O", ![n]),

			IdentifyGraphicSubrepertoire(n) =>
				write!(" M", ![n]),

			InsertLine(n) =>
				write!("L", [n]),

			Justify(ref args) =>
				write!(" F", args.iter()),

			MediaCopy(value) =>
				write!("i", [value]),

			NextPage(n) =>
				write!("U", [n]),

			Presentation(value) =>
				write!(" Z", [value]),

			PageFormat(n) =>
				write!(" J", [n]),

			PrecedingPage(n) =>
				write!("V", [n]),

			PagePosition(n) =>
				write!(" P", [n]),

			PageBack(n) =>
				write!(" R", [n]),

			PageForward(n) =>
				write!(" Q", [n]),

			ParallelText(value) =>
				write!("\\", [value]),

			GraphicDisposition(ref dispositions) =>
				write!(" H", dispositions.iter().map(|&d| Some(Into::<u32>::into(d)))),

			RestoreCursor =>
				write!("u"),

			Repeat(n) =>
				write!("b", [n]),

			Reset(ref modes) =>
				write!("l", modes.iter().map(|&m| Some(Into::<u32>::into(m)))),

			CharacterOrientation(n) =>
				write!(" e", [match n {
					0   => 0u32,
					45  => 1u32,
					90  => 2u32,
					135 => 3u32,
					180 => 4u32,
					225 => 5u32,
					270 => 6u32,
					315 => 7u32,
					_   => unreachable!(),
				}]),

			SaveCursor =>
				write!("s"),

			CharacterSpacing(n) =>
				write!(" b", [n]),

			ScrollDown(n) =>
				write!("T", [n]),

			Movement(direction) =>
				write!("^", [direction]),

			SelectGraphicalRendition(ref attrs) =>
				write!("m", attrs.iter().flat_map(|&a| Into::<Vec<u32>>::into(a)).map(Some)),

			ScrollLeft(n) =>
				write!(" @", [n]),

			LineSpacing(n) =>
				write!(" h", [n]),

			Set(ref modes) =>
				write!("h", modes.iter().map(|&m| Into::<u32>::into(m)).map(Some)),

			ScrollRight(n) =>
				write!(" A", [n]),

			ReverseString(false) =>
				write!("[", [0u32]),

			ReverseString(true) =>
				write!("[", [1u32]),

			SizeUnit(unit) =>
				write!(" I", [unit]),

			SpaceWidth(n) =>
				write!(" [", [n]),

			ScrollUp(n) =>
				write!("S", [n]),

			TabulationClear(value) =>
				write!("g", [value]),

			LinePosition(n) =>
				write!("d", [n]),

			Unknown(id, modifier, ref args) => {
				write!(entry false);
				write!(parameters args.iter());
				write!(identifier id, modifier)
			}

			Private(id, modifier, ref args) => {
				write!(entry true);
				write!(parameters args.iter());
				write!(identifier id, modifier)
			}
		}
	}
}

mod erase;
pub use self::erase::Erase;

mod tabulation;
pub use self::tabulation::Tabulation;

mod qualification;
pub use self::qualification::Qualification;

mod combination;
pub use self::combination::Combination;

mod copy;
pub use self::copy::Copy;

mod expansion;
pub use self::expansion::Expansion;

mod parallel;
pub use self::parallel::Parallel;

mod disposition;
pub use self::disposition::Disposition;

mod mode;
pub use self::mode::Mode;

mod direction;
pub use self::direction::Direction;

mod unit;
pub use self::unit::Unit;

const DIGIT:    &'static [u8] = b"0123456789\x08\x09\x0A\x0B\x0D";
const LETTER:   &'static [u8] = b"@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
const MODIFIER: &'static [u8] = b" !\"#$%&'()*+,-./";

named!(pub parse<CSI>,
	alt!(private | normal));

named!(private<CSI>,
	chain!(
		char!('?') ~
		args:     parameters ~
		modifier: opt!(one_of!(MODIFIER)) ~
		id:       one_of!(LETTER),

		|| Private(id as u8, modifier.map(|c| c as u8), args)));

named!(normal<CSI>,
	chain!(
		args:     parameters ~
		modifier: opt!(one_of!(MODIFIER)) ~
		id:       one_of!(LETTER) ~
		res:      expr_opt!(standard(id, modifier, args)),

		|| res));

named!(parameters<Vec<Option<u32>>>,
	many0!(parameter));

named!(parameter<Option<u32> >,
	alt!(
		char!(';') => { |_| None } |
		chain!(
			number: is_a!(DIGIT) ~
			opt!(char!(';')),

			|| number) => { |n| Some(number(n)) }));

fn number(i: &[u8]) -> u32 {
	let mut n = 0;

	for &ch in i {
		let d = (ch as u32).wrapping_sub(b'0' as u32);

		if d <= 9 {
			n = (n * 10) + d;
		}
	}

	n
}

fn standard(id: char, modifier: Option<char>, args: Vec<Option<u32>>) -> Option<CSI> {
	match (id, modifier) {
		('m', None) => SGR(&args),
		('H', None) => CUP(&args),
		('D', None) => CUB(&args),
		('B', None) => CUD(&args),
		('C', None) => CUF(&args),
		('A', None) => CUU(&args),
		('d', None) => VPA(&args),
		('k', None) => VPB(&args),
		('e', None) => VPR(&args),
		('J', None) => ED(&args),
		('K', None) => EL(&args),
		('@', None) => ICH(&args),
		('L', None) => IL(&args),
		('N', None) => EF(&args),
		('X', None) => ECH(&args),
		('P', None) => DCH(&args),
		('M', None) => DL(&args),

		('Z',  None) => CBT(&args),
		('G',  None) => CHA(&args),
		('I',  None) => CHT(&args),
		('E',  None) => CNL(&args),
		('F',  None) => CPL(&args),
		('R',  None) => CPR(&args),
		('W',  None) => CTC(&args),
		('Y',  None) => CVT(&args),
		('c',  None) => DA(&args),
		('o',  None) => DAQ(&args),
		('n',  None) => DSR(&args),
		('O',  None) => EA(&args),
		('`',  None) => HPA(&args),
		('j',  None) => HPB(&args),
		('a',  None) => HPR(&args),
		('f',  None) => HVP(&args),
		('i',  None) => MC(&args),
		('U',  None) => NP(&args),
		('V',  None) => PP(&args),
		('\\', None) => PTX(&args),
		('u',  None) => RCP(&args),
		('b',  None) => REP(&args),
		('l',  None) => RM(&args),
		('s',  None) => SCP(&args),
		('T',  None) => SD(&args),
		('^',  None) => SIMD(&args),
		('h',  None) => SM(&args),
		('[',  None) => SRS(&args),
		('S',  None) => SU(&args),
		('g',  None) => TBC(&args),

		('T', Some(' ')) => DTA(&args),
		('W', Some(' ')) => FNK(&args),
		('D', Some(' ')) => FNT(&args),
		('_', Some(' ')) => GCC(&args),
		('B', Some(' ')) => GSM(&args),
		('O', Some(' ')) => IDCS(&args),
		('M', Some(' ')) => IGS(&args),
		('F', Some(' ')) => JFY(&args),
		('Z', Some(' ')) => PEC(&args),
		('J', Some(' ')) => PFS(&args),
		('P', Some(' ')) => PPA(&args),
		('R', Some(' ')) => PPB(&args),
		('Q', Some(' ')) => PPR(&args),
		('H', Some(' ')) => QUAD(&args),
		('e', Some(' ')) => SCO(&args),
		('b', Some(' ')) => SCS(&args),
		('@', Some(' ')) => SL(&args),
		('h', Some(' ')) => SLS(&args),
		('A', Some(' ')) => SR(&args),
		('I', Some(' ')) => SSU(&args),
		('[', Some(' ')) => SSW(&args),

		_ =>
			Some(Unknown(id as u8, modifier.map(|m| m as u8), args))
	}
}

with_args!(CBT<1, args> -> CSI,
	CursorBackTabulation(arg!(args[0] => 1)));

with_args!(CHA<1, args> -> CSI,
	CursorHorizontalPosition(arg!(args[0] => 1) - 1));

with_args!(CHT<1, args> -> CSI,
	CursorForwardTabulation(arg!(args[0] => 1)));

with_args!(CNL<1, args> -> CSI,
	CursorNextLine(arg!(args[0] => 1)));

with_args!(CPL<1, args> -> CSI,
	CursorPreviousLine(arg!(args[0] => 1)));

with_args!(CPR<2, args> -> CSI,
	CursorPositionReport { y: arg!(args[0] => 1) - 1, x: arg!(args[1] => 1) - 1 });

with_args!(CTC<1, args> -> CSI, ?
	Tabulation::parse(arg!(args[0] => 0)).map(CursorTabulationControl));

with_args!(CUB<1, args> -> CSI,
	CursorBack(arg!(args[0] => 1)));

with_args!(CUD<1, args> -> CSI,
	CursorDown(arg!(args[0] => 1)));

with_args!(CUF<1, args> -> CSI,
	CursorForward(arg!(args[0] => 1)));

with_args!(CUP<2, args> -> CSI,
	CursorPosition { y: arg!(args[0] => 1) - 1, x: arg!(args[1] => 1) - 1 });

with_args!(CUU<1, args> -> CSI,
	CursorUp(arg!(args[0] => 1)));

with_args!(CVT<1, args> -> CSI,
	CursorLineTabulation(arg!(args[0] => 1)));

with_args!(DA<1, args> -> CSI,
	DeviceAttributes(arg!(args[0] => 0)));

with_args!(DAQ<1, args> -> CSI, ?
	Qualification::parse(arg!(args[0] => 0)).map(DefineAreaQualification));

with_args!(DCH<1, args> -> CSI,
	DeleteCharacter(arg!(args[0] => 1)));

with_args!(DL<1, args> -> CSI,
	DeleteLine(arg!(args[0] => 1)));

with_args!(DSR<1, args> -> CSI, ?
	match arg!(args[0] => 0) {
		6 =>
			Ok(DeviceStatusReport),

		_ =>
			Err(nom::Err::Code::<&[u8], u32>(ErrorKind::Custom(9004)))
	});

with_args!(DTA<2, args> -> CSI,
	DimensionTextArea(arg!(args[0] => 0), arg!(args[1] => 0)));

with_args!(EA<1, args> -> CSI, ?
	Erase::parse(arg!(args[0] => 0)).map(EraseArea));

with_args!(ECH<1, args> -> CSI,
	EraseCharacter(arg!(args[0] => 1)));

with_args!(ED<1, args> -> CSI, ?
	Erase::parse(arg!(args[0] => 0)).map(EraseDisplay));

with_args!(EF<1, args> -> CSI, ?
	Erase::parse(arg!(args[0] => 0)).map(EraseField));

with_args!(EL<1, args> -> CSI, ?
	Erase::parse(arg!(args[0] => 0)).map(EraseLine));

with_args!(FNK<1, args> -> CSI,
	FunctionKey(arg!(args[0] => 0)));

with_args!(FNT<2, args> -> CSI,
	SelectFont(arg!(args[0] => 0), arg!(args[1] => 0)));

with_args!(GCC<1, args> -> CSI, ?
	Combination::parse(arg!(args[0] => 0)).map(GraphicCharacterCombination));

with_args!(GSM<2, args> -> CSI,
	GraphicSizeModification { height: arg!(args[0] => 100), width: arg!(args[1] => 100) });

with_args!(HPA<1, args> -> CSI,
	CursorHorizontalPosition(arg!(args[0] => 1) - 1));

with_args!(HPB<1, args> -> CSI,
	CursorBack(arg!(args[0] => 1)));

with_args!(HPR<1, args> -> CSI,
	CursorForward(arg!(args[0] => 1)));

with_args!(HVP<2, args> -> CSI,
	CursorPosition{ y: arg!(args[0] => 1) - 1, x: arg!(args[1] => 1) - 1 });

with_args!(ICH<1, args> -> CSI,
	InsertCharacter(arg!(args[0] => 1)));

with_args!(IDCS<1, args> -> CSI,
	IdentifyDeviceControlString(arg!(args[0])));

with_args!(IGS<1, args> -> CSI,
	IdentifyGraphicSubrepertoire(arg!(args[0])));

with_args!(IL<1, args> -> CSI,
	InsertLine(arg!(args[0] => 1)));

with_args!(JFY<args> -> CSI,
	Justify(args.to_vec()));

with_args!(MC<1, args> -> CSI, ?
	Copy::parse(arg!(args[0] => 0)).map(MediaCopy));

with_args!(NP<1, args> -> CSI,
	NextPage(arg!(args[0] => 1)));

with_args!(PEC<1, args> -> CSI, ?
	Expansion::parse(arg!(args[0] => 0)).map(Presentation));

with_args!(PFS<1, args> -> CSI,
	PageFormat(arg!(args[0] => 0)));

with_args!(PP<1, args> -> CSI,
	PrecedingPage(arg!(args[0] => 1)));

with_args!(PPA<1, args> -> CSI,
	PagePosition(arg!(args[0] => 1)));

with_args!(PPB<1, args> -> CSI,
	PageBack(arg!(args[0] => 1)));

with_args!(PPR<1, args> -> CSI,
	PageForward(arg!(args[0] => 1)));

with_args!(PTX<1, args> -> CSI, ?
	Parallel::parse(arg!(args[0] => 0)).map(ParallelText));

with_args!(QUAD<args> -> CSI, ?
	args.iter().map(|d| d.unwrap_or(0))
		.map(Disposition::parse)
		.collect::<Result<Vec<_>, _>>()
		.map(GraphicDisposition));

with_args!(RCP -> CSI,
	RestoreCursor);

with_args!(REP<1, args> -> CSI,
	Repeat(arg!(args[0] => 1)));

with_args!(RM<args> -> CSI, ?
	args.iter().map(|d| d.unwrap_or(0))
		.map(Mode::parse)
		.collect::<Result<Vec<_>, _>>()
		.map(Reset));

with_args!(SCO<1, args> -> CSI, ?
	match arg!(args[0] => 0) {
		0 => Ok(CharacterOrientation(0)),
		1 => Ok(CharacterOrientation(45)),
		2 => Ok(CharacterOrientation(90)),
		3 => Ok(CharacterOrientation(135)),
		4 => Ok(CharacterOrientation(180)),
		5 => Ok(CharacterOrientation(225)),
		6 => Ok(CharacterOrientation(270)),
		7 => Ok(CharacterOrientation(315)),
		_ => Err(nom::Err::Code::<&[u8], u32>(ErrorKind::Custom(9002))),
	});

with_args!(SCP -> CSI,
	SaveCursor);

with_args!(SCS<1, args> -> CSI,
	CharacterSpacing(arg!(args[0] => 1)));

with_args!(SD<1, args> -> CSI,
	ScrollDown(arg!(args[0] => 1)));

with_args!(SIMD<1, args> -> CSI, ?
	Direction::parse(arg!(args[0] => 0)).map(Movement));

with_args!(SGR<args> -> CSI, ?
	::SGR::parse(args).map(|v| SelectGraphicalRendition(v)));

with_args!(SL<1, args> -> CSI,
	ScrollLeft(arg!(args[0] => 1)));

with_args!(SLS<1, args> -> CSI,
	LineSpacing(arg!(args[0] => 1)));

with_args!(SM<args> -> CSI, ?
	args.iter().map(|d| d.unwrap_or(0))
		.map(Mode::parse)
		.collect::<Result<Vec<_>, _>>()
		.map(Set));

with_args!(SR<1, args> -> CSI,
	ScrollRight(arg!(args[0] => 1)));

with_args!(SRS<1, args> -> CSI, ?
	match arg!(args[0] => 0) {
		0 => Ok(ReverseString(false)),
		1 => Ok(ReverseString(true)),
		_ => Err(nom::Err::Code::<&[u8], u32>(ErrorKind::Custom(9002))),
	});

with_args!(SSU<1, args> -> CSI, ?
	Unit::parse(arg!(args[0] => 0)).map(SizeUnit));

with_args!(SSW<1, args> -> CSI,
	SpaceWidth(arg!(args[0] => 1)));

with_args!(SU<1, args> -> CSI,
	ScrollUp(arg!(args[0] => 1)));

with_args!(TBC<1, args> -> CSI, ?
	Tabulation::parse(arg!(args[0] => 0)).map(TabulationClear));

with_args!(VPA<1, args> -> CSI,
	LinePosition(arg!(args[0] => 1)));

with_args!(VPB<1, args> -> CSI,
	CursorUp(arg!(args[0] => 1)));

with_args!(VPR<1, args> -> CSI,
	CursorDown(arg!(args[0] => 1)));

pub mod shim {
	pub use super::CSI as T;
	pub use super::CSI::*;
	pub use super::parse;
	pub use super::{Erase, Tabulation, Qualification, Combination, Copy};
	pub use super::{Expansion, Parallel, Disposition, Mode, Direction, Unit};
}

#[cfg(test)]
mod test {
	mod parse {
		use {Control, C1, CSI, parse};

		macro_rules! test {
			($string:expr => $item:expr) => (
				assert_eq!(Control::C1(C1::ControlSequence($item)),
					parse($string).unwrap().1);
			);
		}

		#[test]
		fn cbt() {
			test!(b"\x1B[Z" =>
				CSI::CursorBackTabulation(1));

			test!(b"\x1B[23Z" =>
				CSI::CursorBackTabulation(23));
		}

		#[test]
		fn cha() {
			test!(b"\x1B[G" =>
				CSI::CursorHorizontalPosition(0));

			test!(b"\x1B[43G" =>
				CSI::CursorHorizontalPosition(42));
		}

		#[test]
		fn cht() {
			test!(b"\x1B[I" =>
				CSI::CursorForwardTabulation(1));

			test!(b"\x1B[23I" =>
				CSI::CursorForwardTabulation(23));
		}

		#[test]
		fn cnl() {
			test!(b"\x1B[E" =>
				CSI::CursorNextLine(1));

			test!(b"\x1B[12E" =>
				CSI::CursorNextLine(12));
		}

		#[test]
		fn cpl() {
			test!(b"\x1B[F" =>
				CSI::CursorPreviousLine(1));

			test!(b"\x1B[43F" =>
				CSI::CursorPreviousLine(43));
		}

		#[test]
		fn cpr() {
			test!(b"\x1B[R" =>
				CSI::CursorPositionReport { x: 0, y: 0 });

			test!(b"\x1B[2R" =>
				CSI::CursorPositionReport { x: 0, y: 1 });

			test!(b"\x1B[;2R" =>
				CSI::CursorPositionReport { x: 1, y: 0 });

			test!(b"\x1B[2;2R" =>
				CSI::CursorPositionReport { x: 1, y: 1 });
		}

		#[test]
		fn ctc() {
			test!(b"\x1B[W" =>
				CSI::CursorTabulationControl(CSI::Tabulation::Character));

			test!(b"\x1B[0W" =>
				CSI::CursorTabulationControl(CSI::Tabulation::Character));

			test!(b"\x1B[1W" =>
				CSI::CursorTabulationControl(CSI::Tabulation::Line));

			test!(b"\x1B[2W" =>
				CSI::CursorTabulationControl(CSI::Tabulation::ClearCharacter));

			test!(b"\x1B[3W" =>
				CSI::CursorTabulationControl(CSI::Tabulation::ClearLine));

			test!(b"\x1B[4W" =>
				CSI::CursorTabulationControl(CSI::Tabulation::ClearLineAllCharacters));

			test!(b"\x1B[5W" =>
				CSI::CursorTabulationControl(CSI::Tabulation::ClearAllCharacters));

			test!(b"\x1B[6W" =>
				CSI::CursorTabulationControl(CSI::Tabulation::ClearAllLines));
		}

		#[test]
		fn cub() {
			test!(b"\x1B[D" =>
				CSI::CursorBack(1));

			test!(b"\x1B[37D" =>
				CSI::CursorBack(37));
		}

		#[test]
		fn cud() {
			test!(b"\x1B[B" =>
				CSI::CursorDown(1));

			test!(b"\x1B[42B" =>
				CSI::CursorDown(42));
		}

		#[test]
		fn cuf() {
			test!(b"\x1B[C" =>
				CSI::CursorForward(1));

			test!(b"\x1B[13C" =>
				CSI::CursorForward(13));
		}

		#[test]
		fn cup() {
			test!(b"\x1B[H" =>
				CSI::CursorPosition { x: 0, y: 0 });

			test!(b"\x1B[2;3H" =>
				CSI::CursorPosition { x: 2, y: 1 });

			test!(b"\x1B[;3H" =>
				CSI::CursorPosition{ x: 2, y: 0 });

			test!(b"\x1B[2;H" =>
				CSI::CursorPosition { x: 0, y: 1 });

			test!(b"\x1B[;H" =>
				CSI::CursorPosition { x: 0, y: 0 });
		}

		#[test]
		fn cuu() {
			test!(b"\x1B[A" =>
				CSI::CursorUp(1));

			test!(b"\x1B[23A" =>
				CSI::CursorUp(23));
		}

		#[test]
		fn cvt() {
			test!(b"\x1B[Y" =>
				CSI::CursorLineTabulation(1));

			test!(b"\x1B[23Y" =>
				CSI::CursorLineTabulation(23));
		}

		#[test]
		fn da() {
			test!(b"\x1B[c" =>
				CSI::DeviceAttributes(0));

			test!(b"\x1B[23c" =>
				CSI::DeviceAttributes(23));
		}

		#[test]
		fn daq() {
			test!(b"\x1B[o" =>
				CSI::DefineAreaQualification(CSI::Qualification::UnprotectedUnguarded));

			test!(b"\x1B[0o" =>
				CSI::DefineAreaQualification(CSI::Qualification::UnprotectedUnguarded));

			test!(b"\x1B[1o" =>
				CSI::DefineAreaQualification(CSI::Qualification::ProtectedGuarded));

			test!(b"\x1B[2o" =>
				CSI::DefineAreaQualification(CSI::Qualification::GraphicCharacterInput));

			test!(b"\x1B[3o" =>
				CSI::DefineAreaQualification(CSI::Qualification::NumericInput));

			test!(b"\x1B[4o" =>
				CSI::DefineAreaQualification(CSI::Qualification::AlphabeticInput));

			test!(b"\x1B[5o" =>
				CSI::DefineAreaQualification(CSI::Qualification::AlignLast));

			test!(b"\x1B[6o" =>
				CSI::DefineAreaQualification(CSI::Qualification::ZeroFill));

			test!(b"\x1B[7o" =>
				CSI::DefineAreaQualification(CSI::Qualification::FieldStart));

			test!(b"\x1B[8o" =>
				CSI::DefineAreaQualification(CSI::Qualification::ProtectedUnguarded));

			test!(b"\x1B[9o" =>
				CSI::DefineAreaQualification(CSI::Qualification::SpaceFill));

			test!(b"\x1B[10o" =>
				CSI::DefineAreaQualification(CSI::Qualification::AlignFirst));

			test!(b"\x1B[11o" =>
				CSI::DefineAreaQualification(CSI::Qualification::Reverse));
		}

		#[test]
		fn dch() {
			test!(b"\x1B[P" =>
				CSI::DeleteCharacter(1));

			test!(b"\x1B[8P" =>
				CSI::DeleteCharacter(8));
		}

		#[test]
		fn dl() {
			test!(b"\x1B[M" =>
				CSI::DeleteLine(1));

			test!(b"\x1B[8M" =>
				CSI::DeleteLine(8));
		}

		#[test]
		fn dsr() {
			test!(b"\x1B[6n" =>
				CSI::DeviceStatusReport);
		}

		#[test]
		fn dta() {
			test!(b"\x1B[ T" =>
				CSI::DimensionTextArea(0, 0));

			test!(b"\x1B[1 T" =>
				CSI::DimensionTextArea(1, 0));

			test!(b"\x1B[;1 T" =>
				CSI::DimensionTextArea(0, 1));

			test!(b"\x1B[1;1 T" =>
				CSI::DimensionTextArea(1, 1));
		}

		#[test]
		fn ea() {
			test!(b"\x1B[O" =>
				CSI::EraseArea(CSI::Erase::ToEnd));

			test!(b"\x1B[0O" =>
				CSI::EraseArea(CSI::Erase::ToEnd));

			test!(b"\x1B[1O" =>
				CSI::EraseArea(CSI::Erase::ToStart));

			test!(b"\x1B[2O" =>
				CSI::EraseArea(CSI::Erase::All));
		}

		#[test]
		fn ech() {
			test!(b"\x1B[X" =>
				CSI::EraseCharacter(1));

			test!(b"\x1B[8X" =>
				CSI::EraseCharacter(8));
		}

		#[test]
		fn ed() {
			test!(b"\x1B[J" =>
				CSI::EraseDisplay(CSI::Erase::ToEnd));

			test!(b"\x1B[0J" =>
				CSI::EraseDisplay(CSI::Erase::ToEnd));

			test!(b"\x1B[1J" =>
				CSI::EraseDisplay(CSI::Erase::ToStart));

			test!(b"\x1B[2J" =>
				CSI::EraseDisplay(CSI::Erase::All));
		}

		#[test]
		fn ef() {
			test!(b"\x1B[N" =>
				CSI::EraseField(CSI::Erase::ToEnd));

			test!(b"\x1B[0N" =>
				CSI::EraseField(CSI::Erase::ToEnd));

			test!(b"\x1B[1N" =>
				CSI::EraseField(CSI::Erase::ToStart));

			test!(b"\x1B[2N" =>
				CSI::EraseField(CSI::Erase::All));
		}

		#[test]
		fn el() {
			test!(b"\x1B[K" =>
				CSI::EraseLine(CSI::Erase::ToEnd));

			test!(b"\x1B[0K" =>
				CSI::EraseLine(CSI::Erase::ToEnd));

			test!(b"\x1B[1K" =>
				CSI::EraseLine(CSI::Erase::ToStart));

			test!(b"\x1B[2K" =>
				CSI::EraseLine(CSI::Erase::All));
		}

		#[test]
		fn fnk() {
			test!(b"\x1B[ W" =>
				CSI::FunctionKey(0));

			test!(b"\x1B[13 W" =>
				CSI::FunctionKey(13));
		}

		#[test]
		fn fnt() {
			test!(b"\x1B[ D" =>
				CSI::SelectFont(0, 0));

			test!(b"\x1B[13 D" =>
				CSI::SelectFont(13, 0));

			test!(b"\x1B[;13 D" =>
				CSI::SelectFont(0, 13));

			test!(b"\x1B[13;13 D" =>
				CSI::SelectFont(13, 13));
		}

		#[test]
		fn gcc() {
			test!(b"\x1B[ _" =>
				CSI::GraphicCharacterCombination(CSI::Combination::Next));

			test!(b"\x1B[0 _" =>
				CSI::GraphicCharacterCombination(CSI::Combination::Next));

			test!(b"\x1B[1 _" =>
				CSI::GraphicCharacterCombination(CSI::Combination::Start));

			test!(b"\x1B[2 _" =>
				CSI::GraphicCharacterCombination(CSI::Combination::End));
		}

		#[test]
		fn gsm() {
			test!(b"\x1B[ B" =>
				CSI::GraphicSizeModification { width: 100, height: 100 });

			test!(b"\x1B[13 B" =>
				CSI::GraphicSizeModification { width: 100, height: 13 });

			test!(b"\x1B[;13 B" =>
				CSI::GraphicSizeModification { width: 13, height: 100 });

			test!(b"\x1B[13;13 B" =>
				CSI::GraphicSizeModification { width: 13, height: 13 });
		}

		#[test]
		fn hpa() {
			test!(b"\x1B[`" =>
				CSI::CursorHorizontalPosition(0));

			test!(b"\x1B[2`" =>
				CSI::CursorHorizontalPosition(1));
		}

		#[test]
		fn hpb() {
			test!(b"\x1B[j" =>
				CSI::CursorBack(1));

			test!(b"\x1B[2j" =>
				CSI::CursorBack(2));
		}

		#[test]
		fn hpr() {
			test!(b"\x1B[a" =>
				CSI::CursorForward(1));

			test!(b"\x1B[2a" =>
				CSI::CursorForward(2));
		}

		#[test]
		fn hvp() {
			test!(b"\x1B[f" =>
				CSI::CursorPosition { x: 0, y: 0 });

			test!(b"\x1B[13f" =>
				CSI::CursorPosition { x: 0, y: 12 });

			test!(b"\x1B[;13f" =>
				CSI::CursorPosition { x: 12, y: 0 });

			test!(b"\x1B[13;13f" =>
				CSI::CursorPosition { x: 12, y: 12 });
		}

		#[test]
		fn ich() {
			test!(b"\x1B[@" =>
				CSI::InsertCharacter(1));

			test!(b"\x1B[23@" =>
				CSI::InsertCharacter(23));
		}

		#[test]
		fn idcs() {
			test!(b"\x1B[ O" =>
				CSI::IdentifyDeviceControlString(None));

			test!(b"\x1B[1 O" =>
				CSI::IdentifyDeviceControlString(Some(1)));
		}

		#[test]
		fn igs() {
			test!(b"\x1B[ M" =>
				CSI::IdentifyGraphicSubrepertoire(None));

			test!(b"\x1B[1 M" =>
				CSI::IdentifyGraphicSubrepertoire(Some(1)));
		}

		#[test]
		fn il() {
			test!(b"\x1B[L" =>
				CSI::InsertLine(1));

			test!(b"\x1B[2L" =>
				CSI::InsertLine(2));
		}

		#[test]
		fn jfy() {
			test!(b"\x1B[1;2 F" =>
				CSI::Justify(vec![Some(1), Some(2)]));
		}

		#[test]
		fn mc() {
			test!(b"\x1B[i" =>
				CSI::MediaCopy(CSI::Copy::ToPrimary));

			test!(b"\x1B[0i" =>
				CSI::MediaCopy(CSI::Copy::ToPrimary));

			test!(b"\x1B[1i" =>
				CSI::MediaCopy(CSI::Copy::FromPrimary));

			test!(b"\x1B[2i" =>
				CSI::MediaCopy(CSI::Copy::ToSecondary));

			test!(b"\x1B[3i" =>
				CSI::MediaCopy(CSI::Copy::FromSecondary));

			test!(b"\x1B[4i" =>
				CSI::MediaCopy(CSI::Copy::StopPrimary));

			test!(b"\x1B[5i" =>
				CSI::MediaCopy(CSI::Copy::StartPrimary));

			test!(b"\x1B[6i" =>
				CSI::MediaCopy(CSI::Copy::StopSecondary));

			test!(b"\x1B[7i" =>
				CSI::MediaCopy(CSI::Copy::StartSecondary));
		}

		#[test]
		fn np() {
			test!(b"\x1B[U" =>
				CSI::NextPage(1));

			test!(b"\x1B[2U" =>
				CSI::NextPage(2));
		}

		#[test]
		fn pec() {
			test!(b"\x1B[ Z" =>
				CSI::Presentation(CSI::Expansion::Normal));

			test!(b"\x1B[0 Z" =>
				CSI::Presentation(CSI::Expansion::Normal));

			test!(b"\x1B[1 Z" =>
				CSI::Presentation(CSI::Expansion::Expanded));

			test!(b"\x1B[2 Z" =>
				CSI::Presentation(CSI::Expansion::Condensed));
		}

		#[test]
		fn pfs() {
			test!(b"\x1B[ J" =>
				CSI::PageFormat(0));

			test!(b"\x1B[3 J" =>
				CSI::PageFormat(3));
		}

		#[test]
		fn pp() {
			test!(b"\x1B[V" =>
				CSI::PrecedingPage(1));

			test!(b"\x1B[3V" =>
				CSI::PrecedingPage(3));
		}

		#[test]
		fn ppa() {
			test!(b"\x1B[ P" =>
				CSI::PagePosition(1));

			test!(b"\x1B[3 P" =>
				CSI::PagePosition(3));
		}

		#[test]
		fn ppb() {
			test!(b"\x1B[ R" =>
				CSI::PageBack(1));

			test!(b"\x1B[3 R" =>
				CSI::PageBack(3));
		}

		#[test]
		fn ppr() {
			test!(b"\x1B[ Q" =>
				CSI::PageForward(1));

			test!(b"\x1B[3 Q" =>
				CSI::PageForward(3));
		}

		#[test]
		fn ptx() {
			test!(b"\x1B[\\" =>
				CSI::ParallelText(CSI::Parallel::End));

			test!(b"\x1B[0\\" =>
				CSI::ParallelText(CSI::Parallel::End));

			test!(b"\x1B[1\\" =>
				CSI::ParallelText(CSI::Parallel::Start));

			test!(b"\x1B[2\\" =>
				CSI::ParallelText(CSI::Parallel::StartSupplementary));

			test!(b"\x1B[3\\" =>
				CSI::ParallelText(CSI::Parallel::StartPhoneticJapanese));

			test!(b"\x1B[4\\" =>
				CSI::ParallelText(CSI::Parallel::StartPhoneticChinese));

			test!(b"\x1B[5\\" =>
				CSI::ParallelText(CSI::Parallel::StopPhonetic));
		}

		#[test]
		fn rcp() {
			test!(b"\x1B[u" =>
				CSI::RestoreCursor);
		}

		#[test]
		fn rep() {
			test!(b"\x1B[b" =>
				CSI::Repeat(1));

			test!(b"\x1B[10b" =>
				CSI::Repeat(10));
		}

		#[test]
		fn rm() {
			test!(b"\x1B[1l" =>
				CSI::Reset(vec![CSI::Mode::GuardedAreaTransfer]));

			test!(b"\x1B[2l" =>
				CSI::Reset(vec![CSI::Mode::KeyboardAction]));

			test!(b"\x1B[3l" =>
				CSI::Reset(vec![CSI::Mode::ControlRepresentation]));

			test!(b"\x1B[4l" =>
				CSI::Reset(vec![CSI::Mode::InsertionReplacement]));

			test!(b"\x1B[5l" =>
				CSI::Reset(vec![CSI::Mode::StatusReportTransfer]));

			test!(b"\x1B[6l" =>
				CSI::Reset(vec![CSI::Mode::Erasure]));

			test!(b"\x1B[7l" =>
				CSI::Reset(vec![CSI::Mode::LineEditing]));

			test!(b"\x1B[8l" =>
				CSI::Reset(vec![CSI::Mode::BidirectionalSupport]));

			test!(b"\x1B[9l" =>
				CSI::Reset(vec![CSI::Mode::DeviceComponentSelect]));

			test!(b"\x1B[10l" =>
				CSI::Reset(vec![CSI::Mode::CharacterEditing]));

			test!(b"\x1B[11l" =>
				CSI::Reset(vec![CSI::Mode::PositioningUnit]));

			test!(b"\x1B[12l" =>
				CSI::Reset(vec![CSI::Mode::SendReceive]));

			test!(b"\x1B[13l" =>
				CSI::Reset(vec![CSI::Mode::FormatEffectorAction]));

			test!(b"\x1B[14l" =>
				CSI::Reset(vec![CSI::Mode::FormatEffectorTransfer]));

			test!(b"\x1B[15l" =>
				CSI::Reset(vec![CSI::Mode::MultipleAreaTransfer]));

			test!(b"\x1B[16l" =>
				CSI::Reset(vec![CSI::Mode::TransferTermination]));

			test!(b"\x1B[17l" =>
				CSI::Reset(vec![CSI::Mode::SelectedAreaTransfer]));

			test!(b"\x1B[18l" =>
				CSI::Reset(vec![CSI::Mode::TabulationStop]));

			test!(b"\x1B[21l" =>
				CSI::Reset(vec![CSI::Mode::GraphicRenditionCombination]));

			test!(b"\x1B[22l" =>
				CSI::Reset(vec![CSI::Mode::ZeroDefault]));
		}

		#[test]
		fn sco() {
			test!(b"\x1B[ e" =>
				CSI::CharacterOrientation(0));

			test!(b"\x1B[0 e" =>
				CSI::CharacterOrientation(0));

			test!(b"\x1B[1 e" =>
				CSI::CharacterOrientation(45));

			test!(b"\x1B[2 e" =>
				CSI::CharacterOrientation(90));

			test!(b"\x1B[3 e" =>
				CSI::CharacterOrientation(135));

			test!(b"\x1B[4 e" =>
				CSI::CharacterOrientation(180));

			test!(b"\x1B[5 e" =>
				CSI::CharacterOrientation(225));

			test!(b"\x1B[6 e" =>
				CSI::CharacterOrientation(270));

			test!(b"\x1B[7 e" =>
				CSI::CharacterOrientation(315));
		}

		#[test]
		fn scp() {
			test!(b"\x1B[s" =>
				CSI::SaveCursor);
		}

		#[test]
		fn scs() {
			test!(b"\x1B[ b" =>
				CSI::CharacterSpacing(1));

			test!(b"\x1B[23 b" =>
				CSI::CharacterSpacing(23));
		}

		#[test]
		fn sd() {
			test!(b"\x1B[T" =>
				CSI::ScrollDown(1));

			test!(b"\x1B[73T" =>
				CSI::ScrollDown(73));
		}

		#[test]
		fn simd() {
			test!(b"\x1B[^" =>
				CSI::Movement(CSI::Direction::Forward));

			test!(b"\x1B[0^" =>
				CSI::Movement(CSI::Direction::Forward));

			test!(b"\x1B[1^" =>
				CSI::Movement(CSI::Direction::Backward));
		}

		#[test]
		fn sl() {
			test!(b"\x1B[ @" =>
				CSI::ScrollLeft(1));

			test!(b"\x1B[12 @" =>
				CSI::ScrollLeft(12));
		}

		#[test]
		fn sls() {
			test!(b"\x1B[ h" =>
				CSI::LineSpacing(1));

			test!(b"\x1B[12 h" =>
				CSI::LineSpacing(12));
		}

		#[test]
		fn sm() {
			test!(b"\x1B[1h" =>
				CSI::Set(vec![CSI::Mode::GuardedAreaTransfer]));

			test!(b"\x1B[2h" =>
				CSI::Set(vec![CSI::Mode::KeyboardAction]));

			test!(b"\x1B[3h" =>
				CSI::Set(vec![CSI::Mode::ControlRepresentation]));

			test!(b"\x1B[4h" =>
				CSI::Set(vec![CSI::Mode::InsertionReplacement]));

			test!(b"\x1B[5h" =>
				CSI::Set(vec![CSI::Mode::StatusReportTransfer]));

			test!(b"\x1B[6h" =>
				CSI::Set(vec![CSI::Mode::Erasure]));

			test!(b"\x1B[7h" =>
				CSI::Set(vec![CSI::Mode::LineEditing]));

			test!(b"\x1B[8h" =>
				CSI::Set(vec![CSI::Mode::BidirectionalSupport]));

			test!(b"\x1B[9h" =>
				CSI::Set(vec![CSI::Mode::DeviceComponentSelect]));

			test!(b"\x1B[10h" =>
				CSI::Set(vec![CSI::Mode::CharacterEditing]));

			test!(b"\x1B[11h" =>
				CSI::Set(vec![CSI::Mode::PositioningUnit]));

			test!(b"\x1B[12h" =>
				CSI::Set(vec![CSI::Mode::SendReceive]));

			test!(b"\x1B[13h" =>
				CSI::Set(vec![CSI::Mode::FormatEffectorAction]));

			test!(b"\x1B[14h" =>
				CSI::Set(vec![CSI::Mode::FormatEffectorTransfer]));

			test!(b"\x1B[15h" =>
				CSI::Set(vec![CSI::Mode::MultipleAreaTransfer]));

			test!(b"\x1B[16h" =>
				CSI::Set(vec![CSI::Mode::TransferTermination]));

			test!(b"\x1B[17h" =>
				CSI::Set(vec![CSI::Mode::SelectedAreaTransfer]));

			test!(b"\x1B[18h" =>
				CSI::Set(vec![CSI::Mode::TabulationStop]));

			test!(b"\x1B[21h" =>
				CSI::Set(vec![CSI::Mode::GraphicRenditionCombination]));

			test!(b"\x1B[22h" =>
				CSI::Set(vec![CSI::Mode::ZeroDefault]));
		}

		#[test]
		fn sr() {
			test!(b"\x1B[ A" =>
				CSI::ScrollRight(1));

			test!(b"\x1B[43 A" =>
				CSI::ScrollRight(43));
		}

		#[test]
		fn srs() {
			test!(b"\x1B[[" =>
				CSI::ReverseString(false));

			test!(b"\x1B[0[" =>
				CSI::ReverseString(false));

			test!(b"\x1B[1[" =>
				CSI::ReverseString(true));
		}

		#[test]
		fn ssu() {
			test!(b"\x1B[ I" =>
				CSI::SizeUnit(CSI::Unit::Character));

			test!(b"\x1B[0 I" =>
				CSI::SizeUnit(CSI::Unit::Character));

			test!(b"\x1B[1 I" =>
				CSI::SizeUnit(CSI::Unit::Millimeter));

			test!(b"\x1B[2 I" =>
				CSI::SizeUnit(CSI::Unit::ComputerDecipoint));

			test!(b"\x1B[3 I" =>
				CSI::SizeUnit(CSI::Unit::Decidot));

			test!(b"\x1B[4 I" =>
				CSI::SizeUnit(CSI::Unit::Mil));

			test!(b"\x1B[5 I" =>
				CSI::SizeUnit(CSI::Unit::BasicMeasuringUnit));

			test!(b"\x1B[6 I" =>
				CSI::SizeUnit(CSI::Unit::Micrometer));

			test!(b"\x1B[7 I" =>
				CSI::SizeUnit(CSI::Unit::Pixel));

			test!(b"\x1B[8 I" =>
				CSI::SizeUnit(CSI::Unit::Decipoint));
		}

		#[test]
		fn ssw() {
			test!(b"\x1B[ [" =>
				CSI::SpaceWidth(1));

			test!(b"\x1B[12 [" =>
				CSI::SpaceWidth(12));
		}

		#[test]
		fn su() {
			test!(b"\x1B[S" =>
				CSI::ScrollUp(1));

			test!(b"\x1B[37S" =>
				CSI::ScrollUp(37));
		}

		#[test]
		fn tbc() {
			test!(b"\x1B[g" =>
				CSI::TabulationClear(CSI::Tabulation::Character));

			test!(b"\x1B[0g" =>
				CSI::TabulationClear(CSI::Tabulation::Character));

			test!(b"\x1B[1g" =>
				CSI::TabulationClear(CSI::Tabulation::Line));

			test!(b"\x1B[2g" =>
				CSI::TabulationClear(CSI::Tabulation::ClearCharacter));

			test!(b"\x1B[3g" =>
				CSI::TabulationClear(CSI::Tabulation::ClearLine));

			test!(b"\x1B[4g" =>
				CSI::TabulationClear(CSI::Tabulation::ClearLineAllCharacters));

			test!(b"\x1B[5g" =>
				CSI::TabulationClear(CSI::Tabulation::ClearAllCharacters));

			test!(b"\x1B[6g" =>
				CSI::TabulationClear(CSI::Tabulation::ClearAllLines));
		}

		#[test]
		fn vpa() {
			test!(b"\x1B[d" =>
				CSI::LinePosition(1));

			test!(b"\x1B[42d" =>
				CSI::LinePosition(42));
		}

		#[test]
		fn vpb() {
			test!(b"\x1B[k" =>
				CSI::CursorUp(1));

			test!(b"\x1B[42k" =>
				CSI::CursorUp(42));
		}

		#[test]
		fn vpr() {
			test!(b"\x1B[e" =>
				CSI::CursorDown(1));

			test!(b"\x1B[42e" =>
				CSI::CursorDown(42));
		}
	}

	mod format {
		use {Control, C1, CSI, format, parse};

		macro_rules! test {
			($code:expr) => (
				let item = Control::C1(C1::ControlSequence($code));

				assert_eq!(item, parse(&format(&item, true)).unwrap().1);
				assert_eq!(item, parse(&format(&item, false)).unwrap().1);
			);
		}

		#[test]
		fn parameters() {
			assert_eq!(&b"\x9B1;~"[..],
				&*format(&Control::C1(C1::ControlSequence(
					CSI::Unknown(b'~', None, vec![Some(1), None]))), false));

			assert_eq!(&b"\x9B;1~"[..],
				&*format(&Control::C1(C1::ControlSequence(
					CSI::Unknown(b'~', None, vec![None, Some(1)]))), false));

			assert_eq!(&b"\x9B1;1~"[..],
				&*format(&Control::C1(C1::ControlSequence(
					CSI::Unknown(b'~', None, vec![Some(1), Some(1)]))), false));
		}

		#[test]
		fn cbt() {
			test!(CSI::CursorBackTabulation(1));
			test!(CSI::CursorBackTabulation(23));
		}

		#[test]
		fn cha() {
			test!(CSI::CursorHorizontalPosition(0));
			test!(CSI::CursorHorizontalPosition(42));
		}

		#[test]
		fn cht() {
			test!(CSI::CursorForwardTabulation(1));
			test!(CSI::CursorForwardTabulation(23));
		}

		#[test]
		fn cnl() {
			test!(CSI::CursorNextLine(1));
			test!(CSI::CursorNextLine(12));
		}

		#[test]
		fn cpl() {
			test!(CSI::CursorPreviousLine(1));
			test!(CSI::CursorPreviousLine(43));
		}

		#[test]
		fn cpr() {
			test!(CSI::CursorPositionReport { x: 0, y: 0 });
			test!(CSI::CursorPositionReport { x: 0, y: 1 });
			test!(CSI::CursorPositionReport { x: 1, y: 0 });
			test!(CSI::CursorPositionReport { x: 1, y: 1 });
		}

		#[test]
		fn ctc() {
			test!(CSI::CursorTabulationControl(CSI::Tabulation::Character));
			test!(CSI::CursorTabulationControl(CSI::Tabulation::Line));
			test!(CSI::CursorTabulationControl(CSI::Tabulation::ClearCharacter));
			test!(CSI::CursorTabulationControl(CSI::Tabulation::ClearLine));
			test!(CSI::CursorTabulationControl(CSI::Tabulation::ClearLineAllCharacters));
			test!(CSI::CursorTabulationControl(CSI::Tabulation::ClearAllCharacters));
			test!(CSI::CursorTabulationControl(CSI::Tabulation::ClearAllLines));
		}

		#[test]
		fn cub() {
			test!(CSI::CursorBack(1));
			test!(CSI::CursorBack(37));
		}

		#[test]
		fn cud() {
			test!(CSI::CursorDown(1));
			test!(CSI::CursorDown(42));
		}

		#[test]
		fn cuf() {
			test!(CSI::CursorForward(1));
			test!(CSI::CursorForward(13));
		}

		#[test]
		fn cup() {
			test!(CSI::CursorPosition { x: 0, y: 0 });
			test!(CSI::CursorPosition { x: 1, y: 2 });
			test!(CSI::CursorPosition { x: 2, y: 0 });
			test!(CSI::CursorPosition { x: 0, y: 1 });
		}

		#[test]
		fn cuu() {
			test!(CSI::CursorUp(1));
			test!(CSI::CursorUp(23));
		}

		#[test]
		fn cvt() {
			test!(CSI::CursorLineTabulation(1));
			test!(CSI::CursorLineTabulation(23));
		}

		#[test]
		fn da() {
			test!(CSI::DeviceAttributes(0));
			test!(CSI::DeviceAttributes(23));
		}

		#[test]
		fn daq() {
			test!(CSI::DefineAreaQualification(CSI::Qualification::UnprotectedUnguarded));
			test!(CSI::DefineAreaQualification(CSI::Qualification::ProtectedGuarded));
			test!(CSI::DefineAreaQualification(CSI::Qualification::GraphicCharacterInput));
			test!(CSI::DefineAreaQualification(CSI::Qualification::NumericInput));
			test!(CSI::DefineAreaQualification(CSI::Qualification::AlphabeticInput));
			test!(CSI::DefineAreaQualification(CSI::Qualification::AlignLast));
			test!(CSI::DefineAreaQualification(CSI::Qualification::ZeroFill));
			test!(CSI::DefineAreaQualification(CSI::Qualification::FieldStart));
			test!(CSI::DefineAreaQualification(CSI::Qualification::ProtectedUnguarded));
			test!(CSI::DefineAreaQualification(CSI::Qualification::SpaceFill));
			test!(CSI::DefineAreaQualification(CSI::Qualification::AlignFirst));
			test!(CSI::DefineAreaQualification(CSI::Qualification::Reverse));
		}

		#[test]
		fn dch() {
			test!(CSI::DeleteCharacter(1));
			test!(CSI::DeleteCharacter(8));
		}

		#[test]
		fn dl() {
			test!(CSI::DeleteLine(1));
			test!(CSI::DeleteLine(8));
		}

		#[test]
		fn dsr() {
			test!(CSI::DeviceStatusReport);
		}

		#[test]
		fn dta() {
			test!(CSI::DimensionTextArea(0, 0));
			test!(CSI::DimensionTextArea(1, 0));
			test!(CSI::DimensionTextArea(0, 1));
			test!(CSI::DimensionTextArea(1, 1));
		}

		#[test]
		fn ea() {
			test!(CSI::EraseArea(CSI::Erase::ToEnd));
			test!(CSI::EraseArea(CSI::Erase::ToStart));
			test!(CSI::EraseArea(CSI::Erase::All));
		}

		#[test]
		fn ech() {
			test!(CSI::EraseCharacter(1));
			test!(CSI::EraseCharacter(8));
		}

		#[test]
		fn ed() {
			test!(CSI::EraseDisplay(CSI::Erase::ToEnd));
			test!(CSI::EraseDisplay(CSI::Erase::ToStart));
			test!(CSI::EraseDisplay(CSI::Erase::All));
		}

		#[test]
		fn ef() {
			test!(CSI::EraseField(CSI::Erase::ToEnd));
			test!(CSI::EraseField(CSI::Erase::ToEnd));
			test!(CSI::EraseField(CSI::Erase::ToStart));
			test!(CSI::EraseField(CSI::Erase::All));
		}

		#[test]
		fn el() {
			test!(CSI::EraseLine(CSI::Erase::ToEnd));
			test!(CSI::EraseLine(CSI::Erase::ToEnd));
			test!(CSI::EraseLine(CSI::Erase::ToStart));
			test!(CSI::EraseLine(CSI::Erase::All));
		}

		#[test]
		fn fnk() {
			test!(CSI::FunctionKey(0));
			test!(CSI::FunctionKey(13));
		}

		#[test]
		fn fnt() {
			test!(CSI::SelectFont(0, 0));
			test!(CSI::SelectFont(13, 0));
			test!(CSI::SelectFont(0, 13));
			test!(CSI::SelectFont(13, 13));
		}

		#[test]
		fn gcc() {
			test!(CSI::GraphicCharacterCombination(CSI::Combination::Next));
			test!(CSI::GraphicCharacterCombination(CSI::Combination::Start));
			test!(CSI::GraphicCharacterCombination(CSI::Combination::End));
		}

		#[test]
		fn gsm() {
			test!(CSI::GraphicSizeModification { width: 100, height: 100 });
			test!(CSI::GraphicSizeModification { width: 100, height: 13 });
			test!(CSI::GraphicSizeModification { width: 13, height: 100 });
			test!(CSI::GraphicSizeModification { width: 13, height: 13 });
		}

		#[test]
		fn hpa() {
			test!(CSI::CursorHorizontalPosition(0));
			test!(CSI::CursorHorizontalPosition(1));
		}

		#[test]
		fn hpb() {
			test!(CSI::CursorBack(1));
			test!(CSI::CursorBack(2));
		}

		#[test]
		fn hpr() {
			test!(CSI::CursorForward(1));
			test!(CSI::CursorForward(2));
		}

		#[test]
		fn ich() {
			test!(CSI::InsertCharacter(1));
			test!(CSI::InsertCharacter(23));
		}

		#[test]
		fn idcs() {
			test!(CSI::IdentifyDeviceControlString(None));
			test!(CSI::IdentifyDeviceControlString(Some(1)));
		}

		#[test]
		fn igs() {
			test!(CSI::IdentifyGraphicSubrepertoire(None));
			test!(CSI::IdentifyGraphicSubrepertoire(Some(1)));
		}

		#[test]
		fn il() {
			test!(CSI::InsertLine(1));
			test!(CSI::InsertLine(2));
		}

		#[test]
		fn jfy() {
			test!(CSI::Justify(vec![Some(1), Some(2)]));
		}

		#[test]
		fn mc() {
			test!(CSI::MediaCopy(CSI::Copy::ToPrimary));
			test!(CSI::MediaCopy(CSI::Copy::FromPrimary));
			test!(CSI::MediaCopy(CSI::Copy::ToSecondary));
			test!(CSI::MediaCopy(CSI::Copy::FromSecondary));
			test!(CSI::MediaCopy(CSI::Copy::StopPrimary));
			test!(CSI::MediaCopy(CSI::Copy::StartPrimary));
			test!(CSI::MediaCopy(CSI::Copy::StopSecondary));
			test!(CSI::MediaCopy(CSI::Copy::StartSecondary));
		}

		#[test]
		fn np() {
			test!(CSI::NextPage(1));
			test!(CSI::NextPage(2));
		}

		#[test]
		fn pec() {
			test!(CSI::Presentation(CSI::Expansion::Normal));
			test!(CSI::Presentation(CSI::Expansion::Expanded));
			test!(CSI::Presentation(CSI::Expansion::Condensed));
		}

		#[test]
		fn pfs() {
			test!(CSI::PageFormat(0));
			test!(CSI::PageFormat(3));
		}

		#[test]
		fn pp() {
			test!(CSI::PrecedingPage(1));
			test!(CSI::PrecedingPage(3));
		}

		#[test]
		fn ppa() {
			test!(CSI::PagePosition(1));
			test!(CSI::PagePosition(3));
		}

		#[test]
		fn ppb() {
			test!(CSI::PageBack(1));
			test!(CSI::PageBack(3));
		}

		#[test]
		fn ppr() {
			test!(CSI::PageForward(1));
			test!(CSI::PageForward(3));
		}

		#[test]
		fn ptx() {
			test!(CSI::ParallelText(CSI::Parallel::End));
			test!(CSI::ParallelText(CSI::Parallel::Start));
			test!(CSI::ParallelText(CSI::Parallel::StartSupplementary));
			test!(CSI::ParallelText(CSI::Parallel::StartPhoneticJapanese));
			test!(CSI::ParallelText(CSI::Parallel::StartPhoneticChinese));
			test!(CSI::ParallelText(CSI::Parallel::StopPhonetic));
		}

		#[test]
		fn rcp() {
			test!(CSI::RestoreCursor);
		}

		#[test]
		fn rep() {
			test!(CSI::Repeat(1));
			test!(CSI::Repeat(10));
		}

		#[test]
		fn rm() {
			test!(CSI::Reset(vec![CSI::Mode::GuardedAreaTransfer]));
			test!(CSI::Reset(vec![CSI::Mode::KeyboardAction]));
			test!(CSI::Reset(vec![CSI::Mode::ControlRepresentation]));
			test!(CSI::Reset(vec![CSI::Mode::InsertionReplacement]));
			test!(CSI::Reset(vec![CSI::Mode::StatusReportTransfer]));
			test!(CSI::Reset(vec![CSI::Mode::Erasure]));
			test!(CSI::Reset(vec![CSI::Mode::LineEditing]));
			test!(CSI::Reset(vec![CSI::Mode::BidirectionalSupport]));
			test!(CSI::Reset(vec![CSI::Mode::DeviceComponentSelect]));
			test!(CSI::Reset(vec![CSI::Mode::CharacterEditing]));
			test!(CSI::Reset(vec![CSI::Mode::PositioningUnit]));
			test!(CSI::Reset(vec![CSI::Mode::SendReceive]));
			test!(CSI::Reset(vec![CSI::Mode::FormatEffectorAction]));
			test!(CSI::Reset(vec![CSI::Mode::FormatEffectorTransfer]));
			test!(CSI::Reset(vec![CSI::Mode::MultipleAreaTransfer]));
			test!(CSI::Reset(vec![CSI::Mode::TransferTermination]));
			test!(CSI::Reset(vec![CSI::Mode::SelectedAreaTransfer]));
			test!(CSI::Reset(vec![CSI::Mode::TabulationStop]));
			test!(CSI::Reset(vec![CSI::Mode::GraphicRenditionCombination]));
			test!(CSI::Reset(vec![CSI::Mode::ZeroDefault]));
		}

		#[test]
		fn sco() {
			test!(CSI::CharacterOrientation(0));
			test!(CSI::CharacterOrientation(45));
			test!(CSI::CharacterOrientation(90));
			test!(CSI::CharacterOrientation(135));
			test!(CSI::CharacterOrientation(180));
			test!(CSI::CharacterOrientation(225));
			test!(CSI::CharacterOrientation(270));
			test!(CSI::CharacterOrientation(315));
		}

		#[test]
		fn scp() {
			test!(CSI::SaveCursor);
		}

		#[test]
		fn scs() {
			test!(CSI::CharacterSpacing(1));
			test!(CSI::CharacterSpacing(23));
		}

		#[test]
		fn sd() {
			test!(CSI::ScrollDown(1));
			test!(CSI::ScrollDown(73));
		}

		#[test]
		fn simd() {
			test!(CSI::Movement(CSI::Direction::Forward));
			test!(CSI::Movement(CSI::Direction::Forward));
			test!(CSI::Movement(CSI::Direction::Backward));
		}

		#[test]
		fn sl() {
			test!(CSI::ScrollLeft(1));
			test!(CSI::ScrollLeft(12));
		}

		#[test]
		fn sls() {
			test!(CSI::LineSpacing(1));
			test!(CSI::LineSpacing(12));
		}

		#[test]
		fn sm() {
			test!(CSI::Set(vec![CSI::Mode::GuardedAreaTransfer]));
			test!(CSI::Set(vec![CSI::Mode::KeyboardAction]));
			test!(CSI::Set(vec![CSI::Mode::ControlRepresentation]));
			test!(CSI::Set(vec![CSI::Mode::InsertionReplacement]));
			test!(CSI::Set(vec![CSI::Mode::StatusReportTransfer]));
			test!(CSI::Set(vec![CSI::Mode::Erasure]));
			test!(CSI::Set(vec![CSI::Mode::LineEditing]));
			test!(CSI::Set(vec![CSI::Mode::BidirectionalSupport]));
			test!(CSI::Set(vec![CSI::Mode::DeviceComponentSelect]));
			test!(CSI::Set(vec![CSI::Mode::CharacterEditing]));
			test!(CSI::Set(vec![CSI::Mode::PositioningUnit]));
			test!(CSI::Set(vec![CSI::Mode::SendReceive]));
			test!(CSI::Set(vec![CSI::Mode::FormatEffectorAction]));
			test!(CSI::Set(vec![CSI::Mode::FormatEffectorTransfer]));
			test!(CSI::Set(vec![CSI::Mode::MultipleAreaTransfer]));
			test!(CSI::Set(vec![CSI::Mode::TransferTermination]));
			test!(CSI::Set(vec![CSI::Mode::SelectedAreaTransfer]));
			test!(CSI::Set(vec![CSI::Mode::TabulationStop]));
			test!(CSI::Set(vec![CSI::Mode::GraphicRenditionCombination]));
			test!(CSI::Set(vec![CSI::Mode::ZeroDefault]));
		}

		#[test]
		fn sr() {
			test!(CSI::ScrollRight(1));
			test!(CSI::ScrollRight(43));
		}

		#[test]
		fn srs() {
			test!(CSI::ReverseString(false));
			test!(CSI::ReverseString(true));
		}

		#[test]
		fn ssu() {
			test!(CSI::SizeUnit(CSI::Unit::Character));
			test!(CSI::SizeUnit(CSI::Unit::Millimeter));
			test!(CSI::SizeUnit(CSI::Unit::ComputerDecipoint));
			test!(CSI::SizeUnit(CSI::Unit::Decidot));
			test!(CSI::SizeUnit(CSI::Unit::Mil));
			test!(CSI::SizeUnit(CSI::Unit::BasicMeasuringUnit));
			test!(CSI::SizeUnit(CSI::Unit::Micrometer));
			test!(CSI::SizeUnit(CSI::Unit::Pixel));
			test!(CSI::SizeUnit(CSI::Unit::Decipoint));
		}

		#[test]
		fn ssw() {
			test!(CSI::SpaceWidth(1));
			test!(CSI::SpaceWidth(12));
		}

		#[test]
		fn su() {
			test!(CSI::ScrollUp(1));
			test!(CSI::ScrollUp(37));
		}

		#[test]
		fn tbc() {
			test!(CSI::TabulationClear(CSI::Tabulation::Character));
			test!(CSI::TabulationClear(CSI::Tabulation::Line));
			test!(CSI::TabulationClear(CSI::Tabulation::ClearCharacter));
			test!(CSI::TabulationClear(CSI::Tabulation::ClearLine));
			test!(CSI::TabulationClear(CSI::Tabulation::ClearLineAllCharacters));
			test!(CSI::TabulationClear(CSI::Tabulation::ClearAllCharacters));
			test!(CSI::TabulationClear(CSI::Tabulation::ClearAllLines));
		}

		#[test]
		fn vpa() {
			test!(CSI::LinePosition(1));
			test!(CSI::CursorDown(42));
		}

		#[test]
		fn vpb() {
			test!(CSI::CursorUp(1));
			test!(CSI::CursorUp(42));
		}

		#[test]
		fn vpr() {
			test!(CSI::CursorDown(1));
			test!(CSI::CursorDown(42));
		}
	}
}
