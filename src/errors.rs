#![allow(deprecated)]

error_chain! {
	foreign_links {
		Http(reqwest::Error);
		Json(serde_json::Error);
		Io(std::io::Error);
	}
	errors {
		NotEnoughArgs {
			description("Not enough arguments were passed to the function.")
		}
		TooManyArgs {
			description("Too many arguments were passed to the function.")
		}
	}
}

impl Error {
	pub fn to_error_code(&self) -> u16 {
		// these are arbitrary - 0 is reserved for success
		match self.0 {
			ErrorKind::NotEnoughArgs => 1,
			ErrorKind::TooManyArgs => 2,
			ErrorKind::Http(ref inner_err) if inner_err.is_timeout() => 101,
			ErrorKind::Http(ref inner_err) if inner_err.is_redirect() => 102,
			ErrorKind::Http(_) => 100,
			ErrorKind::Json(_) => 200,
			_ => 99, // unknown
		}
	}
}
