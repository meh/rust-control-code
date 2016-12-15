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

pub trait Format {
	fn fmt<W: Write>(&self, f: W, wide: bool) -> io::Result<()>;
}

#[inline]
pub fn format<T: Format>(value: &T, wide: bool) -> Vec<u8> {
	let mut result = Vec::new();
	value.fmt(&mut result, wide).unwrap();

	result
}

#[inline]
pub fn format_to<T: Format, W: Write>(output: W, value: &T, wide: bool) -> io::Result<()> {
	value.fmt(output, wide)
}
