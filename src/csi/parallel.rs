// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// This file is part of cancer.
//
// cancer is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// cancer is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with cancer.  If not, see <http://www.gnu.org/licenses/>.

use nom;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Parallel {
	End,
	Start,
	StartSupplementary,
	StartPhoneticJapanese,
	StartPhoneticChinese,
	StopPhonetic,
}

impl Parallel {
	pub fn parse<'a>(value: u32) -> Result<Self, nom::Err<&'a [u8]>> {
		match value {
			0 => Ok(Parallel::End),
			1 => Ok(Parallel::Start),
			2 => Ok(Parallel::StartSupplementary),
			3 => Ok(Parallel::StartPhoneticJapanese),
			4 => Ok(Parallel::StartPhoneticChinese),
			5 => Ok(Parallel::StopPhonetic),
			_ => Err(nom::Err::Code(nom::ErrorKind::Custom(9002))),
		}
	}
}


impl Into<u32> for Parallel {
	fn into(self) -> u32 {
		match self {
			Parallel::End                   => 0,
			Parallel::Start                 => 1,
			Parallel::StartSupplementary    => 2,
			Parallel::StartPhoneticJapanese => 3,
			Parallel::StartPhoneticChinese  => 4,
			Parallel::StopPhonetic          => 5,
		}
	}
}
