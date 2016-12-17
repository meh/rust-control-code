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

use nom;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Mode {
	ApplicationCursor,
	Ansi,
	SmallFont,
	SmoothScroll,
	ReverseVideo,
	Origin,
	AutoWrap,
	AutoRepeat,
	FormFeed,
	PrintComplete,
	CursorVisible,
	CursorRTL,
	HebrewKeyboard,
	HebrewEncoding,
	Ascii,
	NorthAmericanKeyboard,
	ProPrinterEmulation,
	VerticalCoupling,
	PageCoupling,
	ApplicationKeypad,
	Backspace,
	DataProcessing,
	SetMargins,
	LimitedTransmission,
	KeyPosition,
	Saver,
	NoClear,
	CopyRTL,
	AutoResize,
	Modem,
	AutoAnswerback,
	ConcealAnswerback,
	IgnoreNull,
	HalfDuplex,
	SecondaryKeyboard,
	Overscan,
	NumLock,
	CapsLock,
	HostIndicatorLed,
}

impl Mode {
	#[inline]
	pub fn parse<'a>(value: u32) -> Result<Self, nom::ErrorKind> {
		match value {
			1   => Ok(Mode::ApplicationCursor),
			2   => Ok(Mode::Ansi),
			3   => Ok(Mode::SmallFont),
			4   => Ok(Mode::SmoothScroll),
			5   => Ok(Mode::ReverseVideo),
			6   => Ok(Mode::Origin),
			7   => Ok(Mode::AutoWrap),
			8   => Ok(Mode::AutoRepeat),
			18  => Ok(Mode::FormFeed),
			19  => Ok(Mode::PrintComplete),
			25  => Ok(Mode::CursorVisible),
			34  => Ok(Mode::CursorRTL),
			35  => Ok(Mode::HebrewKeyboard),
			36  => Ok(Mode::HebrewEncoding),
			42  => Ok(Mode::Ascii),
			57  => Ok(Mode::NorthAmericanKeyboard),
			58  => Ok(Mode::ProPrinterEmulation),
			61  => Ok(Mode::VerticalCoupling),
			64  => Ok(Mode::PageCoupling),
			66  => Ok(Mode::ApplicationKeypad),
			67  => Ok(Mode::Backspace),
			68  => Ok(Mode::DataProcessing),
			69  => Ok(Mode::SetMargins),
			73  => Ok(Mode::LimitedTransmission),
			81  => Ok(Mode::KeyPosition),
			97  => Ok(Mode::Saver),
			95  => Ok(Mode::NoClear),
			96  => Ok(Mode::CopyRTL),
			98  => Ok(Mode::AutoResize),
			99  => Ok(Mode::Modem),
			100 => Ok(Mode::AutoAnswerback),
			101 => Ok(Mode::ConcealAnswerback),
			102 => Ok(Mode::IgnoreNull),
			103 => Ok(Mode::HalfDuplex),
			104 => Ok(Mode::SecondaryKeyboard),
			106 => Ok(Mode::Overscan),
			108 => Ok(Mode::NumLock),
			109 => Ok(Mode::CapsLock),
			110 => Ok(Mode::HostIndicatorLed),

			_ => Err(nom::ErrorKind::Custom(9004)),
		}
	}
}

impl Into<u32> for Mode {
	#[inline]
	fn into(self) -> u32 {
		match self {
			Mode::ApplicationCursor     => 1,
			Mode::Ansi                  => 2,
			Mode::SmallFont             => 3,
			Mode::SmoothScroll          => 4,
			Mode::ReverseVideo          => 5,
			Mode::Origin                => 6,
			Mode::AutoWrap              => 7,
			Mode::AutoRepeat            => 8,
			Mode::FormFeed              => 18,
			Mode::PrintComplete         => 19,
			Mode::CursorVisible         => 25,
			Mode::CursorRTL             => 34,
			Mode::HebrewKeyboard        => 35,
			Mode::HebrewEncoding        => 36,
			Mode::Ascii                 => 42,
			Mode::NorthAmericanKeyboard => 57,
			Mode::ProPrinterEmulation   => 58,
			Mode::VerticalCoupling      => 61,
			Mode::PageCoupling          => 64,
			Mode::ApplicationKeypad     => 66,
			Mode::Backspace             => 67,
			Mode::DataProcessing        => 68,
			Mode::SetMargins            => 69,
			Mode::LimitedTransmission   => 73,
			Mode::KeyPosition           => 81,
			Mode::NoClear               => 95,
			Mode::CopyRTL               => 96,
			Mode::Saver                 => 97,
			Mode::AutoResize            => 98,
			Mode::Modem                 => 99,
			Mode::AutoAnswerback        => 100,
			Mode::ConcealAnswerback     => 101,
			Mode::IgnoreNull            => 102,
			Mode::HalfDuplex            => 103,
			Mode::SecondaryKeyboard     => 104,
			Mode::Overscan              => 106,
			Mode::NumLock               => 108,
			Mode::CapsLock              => 109,
			Mode::HostIndicatorLed      => 110,
		}
	}
}
