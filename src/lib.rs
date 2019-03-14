#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;

mod byond;
mod errors;

use errors::*;
use std::{
	borrow::Cow,
	collections::BTreeMap,
	os::raw::{c_char, c_int},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

lazy_static! {
	static ref HTTP_CLIENT: reqwest::Client = setup_http_client();
}

fn setup_http_client() -> reqwest::Client {
	use reqwest::{
		Client,
		header::{HeaderMap, USER_AGENT}
	};
	let mut headers = HeaderMap::new();
	headers.insert(USER_AGENT, format!("{}/{}", PKG_NAME, VERSION).parse().unwrap());
	Client::builder()
		.default_headers(headers)
		.build()
		.unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn send_post_request(n: c_int, v: *const *const c_char) -> *const i8 {
	let args = byond::parse_args(n, v);
	let res = unwrap_result(send_post_internal(args));
	byond::byond_return(|| Some(res))
}

#[derive(Serialize)]
struct ByondRetVal {
	status_code: u16, // http status code
	error: Option<String>,
	error_code: u16,
	body: Option<String>,
}

fn unwrap_result(err: Result<(String, u16)>) -> String {
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
				error_code: error_code,
				body: None,
			}
		}
	};

	serde_json::to_string(&retval).unwrap()
}

fn send_post_internal(args: Vec<Cow<'_, str>>) -> Result<(String, u16)> {
	if args.len() < 2 {
		return Err(ErrorKind::NotEnoughArgs.into());
	}

	// arg 1 is URL
	// arg 2 is body
	// arg 3 is a JSON map of headers
	let body: String = args[1].as_ref().to_owned();
	let mut req = HTTP_CLIENT.post(&*args[0]).body(body);
	if args.len() > 2 {
		let headers: BTreeMap<&str, &str> = serde_json::from_str(&args[2])?;
		for (key, value) in headers {
			req = req.header(key, value);
		}
	}

	let mut resp = req.send()?;

	let body_resp = resp.text()?;
	let status = resp.status();

	Ok((body_resp, status.as_u16()))
}
