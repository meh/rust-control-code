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
	($args:ident[$index:tt] => $($default:tt)*) => ({
		let default = $($default)* as u32;

		$args.get($index)
			.and_then(|v| v.map(|v| if v == 0 { default } else { v }))
			.unwrap_or(default)
	});

	($args:ident[$index:tt]) => (
		$args.get($index).and_then(|v| *v)
	);
}

macro_rules! with_args {
	($name:ident<$n:tt, $params:ident> -> $ty:ty, ? $body:expr) => (
		fn $name<'a>($params: &[Option<u32>]) -> Option<$ty> {
			if $params.len() <= $n {
				$body.ok()
			}
			else {
				None
			}
		}
	);

	($name:ident<$n:tt, $params:ident> -> $ty:ty, $body:expr) => (
		fn $name<'a>($params: &[Option<u32>]) -> Option<$ty> {
			if $params.len() <= $n {
				Some($body)
			}
			else {
				None
			}
		}
	);

	($name:ident<$params:ident> -> $ty:ty, ? $body:expr) => (
		fn $name<'a>($params: &[Option<u32>]) -> Option<$ty> {
			$body.ok()
		}
	);

	($name:ident<$params:ident> -> $ty:ty, $body:expr) => (
		fn $name<'a>($params: &[Option<u32>]) -> Option<$ty> {
			Some($body)
		}
	);

	($name:ident -> $ty:ty, $body:expr) => (
		fn $name<'a>(args: &[Option<u32>]) -> Option<$ty> {
			if args.is_empty() {
				Some($body)
			}
			else {
				None
			}
		}
	);
}

macro_rules! small_vec {
	() => (
		$crate::smallvec::SmallVec::new()
	);

	($($value:expr),+) => ({
		let mut result = small_vec![];
		$(result.push($value));*;

		result
	});
}
