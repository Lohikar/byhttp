// This file is originally from https://github.com/tgstation/rust-g/blob/master/src/byond.rs, commit 32cc3e7d7605c22173750e4d316ffbb235f59046

use std::{
	borrow::Cow,
	cell::RefCell,
	ffi::{CStr, CString},
	os::raw::{c_char, c_int},
	slice,
};

static EMPTY_STRING: &[c_char; 1] = &[0];
thread_local! {
	static RETURN_STRING: RefCell<CString> = RefCell::new(CString::default());
}

pub fn parse_args<'a>(argc: c_int, argv: *const *const c_char) -> Vec<Cow<'a, str>> {
	unsafe {
		slice::from_raw_parts(argv, argc as usize)
			.iter()
			.map(|ptr| CStr::from_ptr(*ptr))
			.map(|cstr| cstr.to_string_lossy())
			.collect()
	}
}

pub fn byond_return<F, S>(inner: F) -> *const c_char
where
	F: FnOnce() -> Option<S>,
	S: Into<Vec<u8>>,
{
	match inner() {
		Some(string) => RETURN_STRING.with(|cell| {
			let cstring = CString::new(string).expect("null in returned string!");
			cell.replace(cstring);
			cell.borrow().as_ptr() as *const c_char
		}),
		None => EMPTY_STRING as *const c_char,
	}
}
