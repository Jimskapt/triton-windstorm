pub mod data;
pub mod index;
pub mod settings;

pub enum Message {
	Index(index::Message),
	Settings(settings::Message),
	Data(data::Message),

	SaveStorage { key: String, value: String },
	AllowStorage,
	DismissStorageWarning,

	GoToPanel { panel: crate::model::AppPanel },
	UrlChanged(seed::prelude::subs::UrlChanged),
}

pub fn update(
	message: crate::messages::Message,
	model: &mut crate::model::Model,
	orders: &mut impl seed::prelude::Orders<crate::messages::Message>,
) {
	let storage = seed::prelude::web_sys::window()
		.unwrap()
		.local_storage()
		.unwrap()
		.unwrap();

	match message {
		Message::Index(message) => index::update(message, model, orders),
		Message::Settings(message) => settings::update(message, model, orders),
		Message::Data(message) => data::update(message, model, orders),

		Message::SaveStorage { key, value } => {
			model.allowed_save = crate::storage::get_allowed(&storage) == "true";
			model.show_unallowed_save = crate::storage::get_allowed(&storage) != "true";

			if model.allowed_save {
				storage
					.set_item(
						&format!("{}{}", crate::storage::STORAGE_PREFIX, key),
						&value,
					)
					.unwrap();
			} else {
				seed::log!(&format!(
					"can not save `{}` in local storage because user has not allowed it (yet)",
					key
				));
			}
		}
		Message::AllowStorage => {
			model.show_unallowed_save = false;
			model.allowed_save = true;

			storage
				.set_item(
					&format!("{}allowed_save", crate::storage::STORAGE_PREFIX),
					"true",
				)
				.unwrap();
		}
		Message::DismissStorageWarning => {
			model.show_unallowed_save = false;
		}

		Message::GoToPanel { panel } => {
			model.panel = panel;
		}
		Message::UrlChanged(seed::prelude::subs::UrlChanged(url)) => {
			if let Some(path) = url.hash_path().first() {
				if path == "index" {
					orders.send_msg(Message::GoToPanel {
						panel: crate::model::AppPanel::Index,
					});
				} else if path == "settings" {
					orders.send_msg(Message::GoToPanel {
						panel: crate::model::AppPanel::Settings,
					});
				} else if path == "your-data" {
					orders.send_msg(Message::GoToPanel {
						panel: if let Some(path) = url.hash_path().get(1) {
							if path == "import" {
								crate::model::AppPanel::ExportData
							} else {
								crate::model::AppPanel::ExportData
							}
						} else {
							crate::model::AppPanel::ExportData
						},
					});
				}
			}
		}
	}
}
