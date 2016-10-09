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

#[macro_export]
macro_rules! arg {
	($args:ident[$index:tt] => $default:tt) => (
		$args.get($index).and_then(|v| *v).unwrap_or($default)
	);

	($args:ident[$index:tt]) => (
		$args.get($index).and_then(|v| *v)
	);
}

macro_rules! alt_apply {
	($i:expr, $arg:expr; $t:ident $(| $rest:tt)*) => (
		alt!($i, apply!($t, $arg) $(| apply!($rest, $arg))*)
	);
}
