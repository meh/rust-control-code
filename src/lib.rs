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

#![allow(non_snake_case)]
#![recursion_limit="100"]

#[macro_use]
pub extern crate nom;
pub use nom::IResult as Result;

#[macro_use]
mod util;

mod control;
pub use control::{Control, parse};

mod format;
pub use format::{Format, format, format_to};

mod c0;
pub use c0::shim as C0;

mod c1;
pub use c1::shim as C1;

mod csi;
pub use csi::shim as CSI;

mod sgr;
pub use sgr::shim as SGR;
