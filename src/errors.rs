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
	#[error("ureq error: {0}")]
	Ureq(#[from] ureq::Error),
	#[error("Invalid URI: {0}")]
	InvalidUri(#[from] http::uri::InvalidUri)
}

impl ByError {
	pub fn to_error_code(&self) -> u16 {
		// these are arbitrary - 0 is reserved for success
		// Should not exceed a u24 due to BYOND.
		match self {
			Self::NotEnoughArgs => 1,
			Self::TooManyArgs => 2,
			Self::InvalidUri(_) => 3,
			Self::Ureq(ureq::Error::Timeout(_t)) => 101,	// timeout
			Self::Ureq(ureq::Error::TooManyRedirects) => 102,
			Self::Ureq(..) => 99,
			Self::Json { .. } => 200,
		}
	}
}
