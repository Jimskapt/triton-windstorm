#![allow(clippy::needless_return)]
#![allow(clippy::wildcard_imports)]

use seed::prelude::*;

mod locale;
mod message;
mod model;
mod storage;
mod view;

#[wasm_bindgen(start)]
pub fn start() {
	if cfg!(debug_assertions) {
		console_error_panic_hook::set_once();
	}

	seed::App::start("app", model::init, message::update, view::view);
}
