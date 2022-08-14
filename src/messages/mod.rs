pub mod data;
pub mod database;
pub mod graphs;
pub mod index;
pub mod settings;

pub enum Message {
	Index(index::Message),
	Settings(settings::Message),
	Data(data::Message),
	Graphs(graphs::Message),
	Database(database::Message),

	SaveStorage { key: String, value: String },
	DeleteStorage(String),
	AllowStorage,
	DismissStorageWarning,

	GoToPanel { panel: crate::model::AppPanel },
	UrlChanged(seed::prelude::subs::UrlChanged),

	BatchMessages(Vec<Message>),
	None,
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
		Message::Graphs(message) => graphs::update(message, model, orders),
		Message::Database(message) => database::update(message, model, orders),

		Message::SaveStorage { key, value } => {
			model.allowed_save = crate::storage::get_allowed(&storage) == "true";
			model.show_unallowed_save = crate::storage::get_allowed(&storage) != "true";

			let promise = if let Some(remote) = &model.database_remote {
				if remote.is_connected() {
					let (path, content_type) = match key.strip_prefix("record_") {
						Some(date) => (
							format!("/triton_windstorm/records/{date}.json"),
							"application/json",
						),
						None => match key.strip_prefix("subject_") {
							Some(date) => (
								format!("/triton_windstorm/subjects/{date}.json"),
								"application/json",
							),
							None => (format!("/triton_windstorm/{key}"), "text/plain"),
						},
					};

					match remote.put(
						&pontus_onyx::item::ItemPath::from(path.as_str()),
						&pontus_onyx::item::Item::Document {
							etag: "*".into(),
							content: Some(value.as_bytes().to_vec()),
							content_type: content_type.into(),
							last_modified: Some(
								time::OffsetDateTime::parse(
									&chrono::Utc::now().to_rfc2822(),
									&time::format_description::well_known::Rfc2822,
								)
								.unwrap(),
							),
						},
					) {
						Ok(res) => Some(res),
						Err(err) => {
							seed::error!(err);
							None
						}
					}
				} else {
					None
				}
			} else {
				None
			};

			if model.allowed_save {
				let res = storage.set_item(
					&format!("{}{}", crate::storage::STORAGE_PREFIX, key),
					&value,
				);

				if res.is_ok() {
					if let Some(promise) = promise {
						orders.skip().perform_cmd(async move {
							let resp = seed::JsFuture::from(promise).await.unwrap();

							let doc = resp.into_serde::<pontus_onyx::item::Item>().unwrap();

							if let pontus_onyx::item::Item::Document {
								etag,
								last_modified,
								..
							} = doc
							{
								storage
									.set_item(
										&format!("{}etag_{key}", crate::storage::STORAGE_PREFIX),
										&format!("{etag}"),
									)
									.ok();

								if let Some(last_modified) = last_modified {
									if let Ok(last_modified) = last_modified
										.format(&time::format_description::well_known::Rfc3339)
									{
										storage
											.set_item(
												&format!(
													"{}last_modified_{key}",
													crate::storage::STORAGE_PREFIX
												),
												&last_modified,
											)
											.ok();
									}
								} else {
									storage
										.remove_item(&format!(
											"{}last_modified_{key}",
											crate::storage::STORAGE_PREFIX
										))
										.ok();
								}
							}
						});
					} else {
						storage
							.remove_item(&format!("{}etag_{key}", crate::storage::STORAGE_PREFIX))
							.ok();

						storage
							.remove_item(&format!(
								"{}last_modified_{key}",
								crate::storage::STORAGE_PREFIX
							))
							.ok();
					}
				}
			} else {
				seed::log!(&format!(
					"can not save `{}` in local storage because user has not allowed it (yet)",
					key
				));

				storage
					.remove_item(&format!("{}etag_{key}", crate::storage::STORAGE_PREFIX))
					.ok();

				storage
					.remove_item(&format!(
						"{}last_modified_{key}",
						crate::storage::STORAGE_PREFIX
					))
					.ok();
			}
		}
		Message::DeleteStorage(key) => {
			model.allowed_save = crate::storage::get_allowed(&storage) == "true";
			model.show_unallowed_save = crate::storage::get_allowed(&storage) != "true";

			if model.allowed_save {
				storage
					.remove_item(&format!("{}{}", crate::storage::STORAGE_PREFIX, key))
					.unwrap();
			} else {
				seed::log!(&format!(
					"can not delete `{}` in local storage because user has not allowed it (yet)",
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
			if let crate::model::AppPanel::Graphics = panel {
				model.do_render_graphics = true;
				orders.send_msg(crate::messages::Message::Graphs(
					crate::messages::graphs::Message::ComputeHistoricalSubjects,
				));
				orders.after_next_render(|_| {
					crate::messages::Message::Graphs(crate::messages::graphs::Message::UpdateGraph)
				});
			} else {
				model.do_render_graphics = false;
			}

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
								crate::model::AppPanel::ImportData
							} else {
								crate::model::AppPanel::ExportData
							}
						} else {
							crate::model::AppPanel::ExportData
						},
					});
				} else if path == "graphs" {
					orders.send_msg(Message::GoToPanel {
						panel: crate::model::AppPanel::Graphics,
					});
				} else if path == "about" {
					orders.send_msg(Message::GoToPanel {
						panel: crate::model::AppPanel::About,
					});
				}
			}
		}
		Message::BatchMessages(messages) => {
			for message in messages {
				orders.send_msg(message);
			}
		}
		Message::None => {}
	}
}
