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

use DEC::T as DEC;
use DEC::*;

pub fn normal<'a>(id: u8, modifier: Option<u8>, args: &[Option<u32>]) -> Option<DEC<'a>> {
	match (id, modifier) {
		(b's', None) => SCOSC(args),
		(b'u', None) => SCORC(args),
		(b'r', None) => DECSTBM(args),

		(b'q', Some(b' ')) => DECSCUSR(args),
		(b'p', Some(b'!')) => DECSTR(args),

		_ => None
	}
}

pub fn private<'a>(id: u8, modifier: Option<u8>, args: &[Option<u32>]) -> Option<DEC<'a>> {
	match (id, modifier) {
		(b'h', None) => SM(args),
		(b'l', None) => RM(args),

		(b'~', Some(b'\'')) => DECDC(args),
		(b'}', Some(b'\'')) => DECIC(args),

		_ => None
	}
}

with_args!(SCOSC -> DEC<'a>,
	SaveCursorPosition);

with_args!(SCORC -> DEC<'a>,
	RestoreCursorPosition);

with_args!(SM<args> -> DEC<'a>, ?
	args.iter().map(|d| d.unwrap_or(0))
		.map(Mode::parse)
		.collect::<Result<Vec<_>, _>>()
		.map(Set));

with_args!(RM<args> -> DEC<'a>, ?
	args.iter().map(|d| d.unwrap_or(0))
		.map(Mode::parse)
		.collect::<Result<Vec<_>, _>>()
		.map(Reset));

with_args!(DECDC<1, args> -> DEC<'a>,
	DeleteColumn(arg!(args[0] => 1)));

with_args!(DECIC<1, args> -> DEC<'a>,
	InsertColumn(arg!(args[0] => 1)));

with_args!(DECSCUSR<1, args> -> DEC<'a>,
	CursorStyle(arg!(args[0] => 0) as u8));

with_args!(DECSTBM<2, args> -> DEC<'a>,
	ScrollRegion { top: arg!(args[0] => 1) - 1, bottom: arg!(args[1]).map(|b| b - 1) });

with_args!(DECSTR -> DEC<'a>,
	SoftReset);

// TODO: DECCARA
// TODO: DECCRA
// TODO: DECELF
// TODO: DECERA
// TODO: DECFNK
// TODO: DECFRA
// TODO: DECINVM
// TODO: DECKBD
// TODO: DECLFKC
// TODO: DECLL
// TODO: DECLTOD
// TODO: DECPCTERM
// TODO: DECPKA
// TODO: DECPKFMR
// TODO: DECRARA
// TODO: DECRPDE
// TODO: DECRPKT
// TODO: DECRPM
// TODO: DECRQCRA
// TODO: DECRQDE
// TODO: DECRQKD
// TODO: DECRQKT
// TODO: DECRQM
// TODO: DECRQPKFM
// TODO: DECRQPSR
// TODO: DECRQTSR
// TODO: DECRQUPSS
// TODO: DECSACE
// TODO: DECSASD
// TODO: DECSCA
// TODO: DECSCL
// TODO: DECSCP
// TODO: DECSCPP
// TODO: DECSCS
// TODO: DECSDDT
// TODO: DECSDPT
// TODO: DECSED
// TODO: DECSEL
// TODO: DECSERA
// TODO: DECSFC
// TODO: DECSKCV
// TODO: DECSLCK
// TODO: DECSLPP
// TODO: DECSLRM
// TODO: DECSMBV
// TODO: DECSMKR
// TODO: DECSNLS
// TODO: DECSPP
// TODO: DECSPPCS
// TODO: DECSPRTT
// TODO: DECSR
// TODO: DECSRC
// TODO: DECSRFR
// TODO: DECSSCLS
// TODO: DECSSDT
// TODO: DECSSL
// TODO: DECST8C
// TODO: DECSTRL
// TODO: DECSWBV
// TODO: DECTID
// TODO: DECTME
// TODO: DECTSR
// TODO: DECTST
// TODO: DECXCPR
// TODO: DSR-CPR
// TODO: DSR-DIR
// TODO: DSR-XCPR
// TODO: DSR-KBD
// TODO: DSR-MSR
// TODO: DSR-DECCKSR
// TODO: DSR-OS
// TODO: DSR-PP
// TODO: DSR-UDK
