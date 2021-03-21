#![allow(clippy::needless_return)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::struct_excessive_bools)]

use seed::prelude::*;

mod locale;
mod messages;
mod model;
mod storage;
mod view;

#[wasm_bindgen(start)]
pub fn start() {
	if cfg!(debug_assertions) {
		console_error_panic_hook::set_once();
	}

	seed::App::start("app", model::init, messages::update, view::view);
}
