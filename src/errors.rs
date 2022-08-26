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
	#[error("Body too long to decode (>10MB).")]
	BodyTooLarge,
}

impl ByError {
	pub fn to_error_code(&self) -> u16 {
		// these are arbitrary - 0 is reserved for success
		// Should not exceed a u24 due to BYOND.
		match self {
			Self::NotEnoughArgs => 1,
			Self::TooManyArgs => 2,
			Self::Ureq(ureq::Error::Transport(t)) if t.kind() == ureq::ErrorKind::Io => 101,	// timeout
			Self::Ureq(ureq::Error::Transport(t)) if t.kind() == ureq::ErrorKind::TooManyRedirects => 102,
			Self::Ureq(..) => 99,
			Self::Json { .. } => 200,
			Self::BodyTooLarge => 201,
		}
	}
}
