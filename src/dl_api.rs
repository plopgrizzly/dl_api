// dl_api
//
// Copyright (c) 2018-2019 Jeron Aldaron Lau
// Copyright (c) 2017 Szymon Wieloch
// Distributed under the MIT LICENSE (See accompanying file LICENSE.txt)

/// Macro to generate the API struct.
///
/// ```
/// dl_api::link!(MyApi, "libmylibrary.so.1", {
/// 	fn cFunction(param_name: ParamType) -> ReturnType;
/// });
///
/// fn main() {
/// 	let api = MyApi::new().unwrap(); // unwrap the `Result`.
///
/// 	let rtn: ReturnType = unsafe {
/// 		(api.cFunction)(0);
/// 	};
/// }
/// ```
#[macro_export]
macro_rules! link(
	($sname: ident, $l: expr, { $(fn $fname: ident($($sarg: ident: $farg: ty),* $(,)?) -> $fret:ty);* $(;)? }) =>
	(
		#[allow(non_snake_case)]
		pub struct $sname {
			#[allow(dead_code)]
			// this is not dead code because destructor of Library
			// deallocates the library
			__lib: $crate::Library,
			$(
				$fname: unsafe extern "system" fn($($farg),*)
					-> $fret,
			)*
		}

		impl $sname {
			fn new() -> ::std::result::Result<Self, $crate::Error> {
				unsafe {
				let __lib = $crate::Library::new($l)?;
				Ok(Self{
					$($fname: {
						match __lib.symbol(stringify!($fname)) {
							Ok(s) => s,
							Err(e) => return Err(e),
						}
					},
					)*
					__lib,
				})
				}
			}
		}
	)
);
