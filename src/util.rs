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
