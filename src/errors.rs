use thiserror::Error;

#[derive(Error, Debug)]
pub enum ByError {
	#[error("Not enough arguments were passed to the function.")]
	NotEnoughArgs,
	#[error("Too many arguments were passed to the function.")]
	TooManyArgs,
	#[error("{source}")]
	Json {
		#[from]
		source: serde_json::Error
	},
	#[error("{source}")]
	Http {
		#[from]
		source: reqwest::Error
	}
}

impl ByError {
	pub fn to_error_code(&self) -> u16 {
		// these are arbitrary - 0 is reserved for success
		match self {
			Self::NotEnoughArgs => 1,
			Self::TooManyArgs => 2,
			Self::Http { ref source } if source.is_timeout() => 101,
			Self::Http { ref source } if source.is_redirect() => 102,
			Self::Http { .. } => 100,
			Self::Json { .. } => 200
		}
	}
}
