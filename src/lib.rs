#![allow(clippy::missing_safety_doc)]

#[macro_use]
extern crate serde_derive;

mod byond;
mod errors;

use errors::*;
use std::{
	borrow::Cow,
	collections::BTreeMap,
	os::raw::{c_char, c_int},
	time::Duration,
};
use once_cell::sync::Lazy;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

static HTTP_AGENT: Lazy<ureq::Agent> = Lazy::new(construct_agent);

fn construct_agent() -> ureq::Agent {
	ureq::AgentBuilder::new()
		.timeout(Duration::from_secs(1))
		.user_agent(&format!("{}/{}", PKG_NAME, VERSION))
		.max_idle_connections(10)
		.build()
}

#[no_mangle]
pub unsafe extern "C" fn send_post_request(n: c_int, v: *const *const c_char) -> *const i8 {
	let args = byond::parse_args(n, v);
	let res = unwrap_result(send_post_internal(args));
	byond::byond_return(|| Some(res))
}

#[no_mangle]
pub unsafe extern "C" fn send_get_request(n: c_int, v: *const *const c_char) -> *const i8 {
	let args = byond::parse_args(n, v);
	let res = unwrap_result(send_get_internal(args));
	byond::byond_return(|| Some(res))
}

#[derive(Serialize)]
struct ByondRetVal {
	status_code: u16, // http status code
	error: Option<String>,
	error_code: u16,
	body: Option<String>,
}

fn unwrap_result(err: Result<(String, u16), ByError>) -> String {
	let retval = match err {
		Ok((body, status_code)) => ByondRetVal {
			status_code,
			error: None,
			error_code: 0,
			body: Some(body),
		},
		Err(err) => {
			let error_code = err.to_error_code();
			eprintln!("byhttp error: {}", &err);
			ByondRetVal {
				status_code: 0,
				error: Some(err.to_string()),
				error_code,
				body: None,
			}
		}
	};

	serde_json::to_string(&retval).unwrap()
}

fn send_post_internal(args: Vec<Cow<'_, str>>) -> Result<(String, u16), ByError> {
	if args.len() < 2 {
		return Err(ByError::NotEnoughArgs);
	}

	// arg 1 is URL
	// arg 2 is body
	// arg 3 is a JSON map of headers
	let body: String = args[1].as_ref().to_owned();

	let mut req = HTTP_AGENT.post(&args[0]);

	match args.len() {
		3 => {
			let headers: BTreeMap<&str, &str> = serde_json::from_str(&args[2])?;
			for (key, value) in headers {
				req = req.set(key, value);
			}
		},
		4.. => return Err(ByError::TooManyArgs),
		_ => unreachable!()
	}

	let response = req.send_string(&body)?;
	let status = response.status();
	let body = response.into_string().map_err(|_| ByError::BodyTooLarge)?;

	Ok((body, status))
}

fn send_get_internal(args: Vec<Cow<'_, str>>) -> Result<(String, u16), ByError> {
	if args.is_empty() {
		return Err(ByError::NotEnoughArgs);
	}

	// arg 0 is URL
	// arg 1 is a JSON map of headers
	let mut req = HTTP_AGENT.get(&args[0]);

	match args.len() {
		2 => {
			let headers: BTreeMap<&str, &str> = serde_json::from_str(&args[1])?;
			for (key, value) in headers {
				req = req.set(key, value);
			}
		},
		3.. => return Err(ByError::TooManyArgs),
		_ => unreachable!(),
	}

	let response = req.call()?;
	let status = response.status();
	let body = response.into_string().map_err(|_| ByError::BodyTooLarge)?;

	Ok((body, status))
}
