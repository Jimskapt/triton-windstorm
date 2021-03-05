pub const STORAGE_PREFIX: &str = "triton_windstorm_";

pub fn get_allowed(storage: &seed::prelude::web_sys::Storage) -> String {
	return storage
		.get_item(&format!("{}allowed_save", STORAGE_PREFIX))
		.unwrap_or_else(|_| Some(String::from("false")))
		.unwrap_or_else(|| String::from("false"));
}

pub fn get_dark(storage: &seed::prelude::web_sys::Storage) -> String {
	return storage
		.get_item(&format!("{}dark_theme", STORAGE_PREFIX))
		.unwrap_or_else(|_| Some(String::from("false")))
		.unwrap_or_else(|| String::from("false"));
}
