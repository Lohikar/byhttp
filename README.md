# ByHTTP

ByHTTP is a basic HTTP POST library for BYOND, meant to be a cross-platform replacement for ByondPOST.

## Compiling

Requirements:

- Cargo
- Rust 1.32 stable

BYOND is 32-bit on all platforms, so you must cross-compile this library to 32-bit if you're on a 64-bit system. If you're on a 32-bit system, you can just compile with `cargo build --release`.

Windows 64-bit cross-compile:

```cmd
> rustup target add i686-pc-windows-msvc
> cargo build --release --target i686-pc-windows-msvc
```

Linux 64-bit cross-compile:

```sh
$ rustup target add i686-unknown-linux-gnu
$ cargo build --release --target i686-unknown-linux-gnu
```

## Interface

ByHTTP exposes the following function(s) - fields with ? are optional:

- `send_post_request(url, payload, headers?)`

Function calls will return a JSON object containing the result of the request, in one of the two forms:

### Protocol Error

This is returned if the input to the function was invalid, or if ByHTTP failed to make the request.

```json
{
    "status_code": 0,
    "error": "Friendly error msg",
    "error_code": 100,
    "body": null
}
```

`status_code` and `body` will be `0` and `null` respectively; `error` will contain a 'friendly' error msg suitable for display in logs.

`error` will contain one of the following error codes:

- `1` - Too few arguments passed to function.
- `2` - Too many arguments passed to function.
- `100` - Unknown HTTP error.
- `101` - Connection timed out or failed to connect.
- `102` - Too many redirects.
- `200` - Error decoding or encoding JSON.
- `99` - Unknown error.

### Success / HTTP Error

This is returned if ByHTTP was able to make the request, even if the response was an HTTP error.

```json
{
    "status_code": 200,
    "error": null,
    "error_code": 0,
    "body": "body content as UTF-8"
}
```

`status_code` will contain the HTTP status code (this may be an error value such as 404), and `body` will contain the response body as UTF-8 encoded text. If an empty body was returned, `body` will be a zero-length string.
