use seed::prelude::JsCast;

static RECORD_NAME_DATE: &str = "[0-9]{4}-[0-9]{2}-[0-9]{2}";

pub enum Message {
	SetDataAccountString(web_sys::Event),

	BuildRemote(Option<Box<super::Message>>),
	SetRemote(
		Result<pontus_onyx::client::ClientRemote, seed::prelude::JsValue>,
		Option<Box<super::Message>>,
	),
	DropRemote,

	SyncDatabase,
	FetchAndSaveRecordFromRemote(String),
	FetchAndSaveSubjectFromRemote(String),
}

pub fn update(
	message: Message,
	model: &mut crate::model::Model,
	orders: &mut impl seed::prelude::Orders<crate::messages::Message>,
) {
	match message {
		Message::SetDataAccountString(event) => {
			let target = event.target().unwrap();
			let target: &web_sys::HtmlInputElement = target.dyn_ref().unwrap();
			let field_value = target.value();

			model.database_account_string = field_value;
		}
		Message::BuildRemote(callback) => {
			let (data_root_uri, data_account) = {
				let separator = '@';
				let input = model.database_account_string.clone();
				let find = input.rfind(separator);

				if let Some(find) = find {
					(
						Some(String::from(&input[(find + 1)..])),
						String::from(&input[..find]),
					)
				} else {
					(None, input)
				}
			};

			if let Some(data_root_uri) = data_root_uri {
				orders.skip().perform_cmd(async move {
					let window = web_sys::window().ok_or("window not found").unwrap();

					let remote: Result<pontus_onyx::client::ClientRemote, seed::prelude::JsValue> =
						pontus_onyx::client::ClientRemote::new(
							data_root_uri.clone(),
							data_account.clone(),
							pontus_onyx::scope::Scope::try_from("triton_windstorm:rw").unwrap(),
							window.location().origin().unwrap(),
							false,
						)
						.await;

					if let Ok(client_remote) = &remote {
						if !client_remote.is_connected() {
							let mut db_account = data_account.clone();
							db_account += "@";
							db_account += &data_root_uri;
							db_account = seed::Url::encode_uri_component(db_account);

							client_remote
								.show_connect_overlay(format!(
									"/db_register?db_account={db_account}"
								))
								.await
								.unwrap();

							crate::messages::Message::None
						} else {
							crate::messages::Message::Database(Message::SetRemote(remote, callback))
						}
					} else {
						crate::messages::Message::None
					}
				});
			}
		}
		Message::SetRemote(res, callback) => {
			if let Ok(remote) = res {
				model.database_remote = Some(remote);

				if let Some(callback) = callback {
					orders.send_msg(*callback);
				}
			}
		}
		Message::DropRemote => {
			model.database_remote = None;
		}
		Message::SyncDatabase => {
			let storage = seed::prelude::web_sys::window()
				.unwrap()
				.local_storage()
				.unwrap()
				.unwrap();

			if let Ok(Some(allowed_save)) =
				storage.get(&format!("{}allowed_save", crate::storage::STORAGE_PREFIX))
			{
				if allowed_save == "true" {
					let mut saved_subjects: Vec<String> = vec![];

					// send unsaved records changes from localStorage in remoteStorage.
					// a record change is detected if the record has no ETag in localStorage.
					if let Ok(length) = storage.length() {
						for i in 0..length {
							let key = storage.key(i).unwrap().unwrap();
							let value = storage.get(&key).unwrap().unwrap();

							if let Some(temp_next) = str::strip_prefix(
								&key,
								&format!("{}subject_", crate::storage::STORAGE_PREFIX),
							) {
								if let Ok(None) = storage.get(&format!(
									"{}etag_subject_{temp_next}",
									crate::storage::STORAGE_PREFIX
								)) {
									match serde_json::from_str::<crate::model::Subject>(&value) {
										Ok(value) => {
											match model.database_remote.as_ref().unwrap().put(
												&pontus_onyx::item::ItemPath::from(
													format!(
														"/triton_windstorm/subjects/{}.json",
														value.id,
													)
													.as_str(),
												),
												&pontus_onyx::item::Item::Document {
													etag: "*".into(),
													content: Some(serde_json::to_string(&value).unwrap().as_bytes().to_vec()),
													content_type: "text/plain".into(),
													last_modified: Some(time::OffsetDateTime::parse(
														&chrono::Utc::now().to_rfc2822(),
														&time::format_description::well_known::Rfc2822,
													).unwrap()),
												},
											) {
												Ok(promise) => {
													orders.skip().perform_cmd(async move {
														let resp = seed::JsFuture::from(promise)
															.await;

														match resp {
															Ok(resp) => {
																let doc = resp
																	.into_serde::<pontus_onyx::item::Item>()
																	.unwrap();

																if let pontus_onyx::item::Item::Document {
																	etag,
																	last_modified,
																	..
																} = doc
																{
																	let storage =
																		seed::prelude::web_sys::window()
																			.unwrap()
																			.local_storage()
																			.unwrap()
																			.unwrap();

																	storage
																		.set_item(
																			&format!(
																				"{}etag_subject_{}",
																				crate::storage::STORAGE_PREFIX,
																				value.id,
																			),
																			&format!("{etag}"),
																		)
																		.ok();

																	if let Some(last_modified) = last_modified {
																		if let Ok(last_modified) = last_modified.format(&time::format_description::well_known::Rfc3339) {
																			storage
																				.set_item(
																					&format!(
																						"{}last_modified_subject_{}",
																						crate::storage::STORAGE_PREFIX,
																						value.id,
																					),
																					&last_modified,
																				)
																				.ok();
																		}
																	} else {
																		storage
																			.remove_item(&format!(
																				"{}last_modified_subject_{}",
																				crate::storage::STORAGE_PREFIX,
																				value.id,
																			))
																			.ok();
																	}
																}
															},
															Err(err) => {
																seed::error!(err);
															}
														}
													});
												},
												Err(err) => {
													seed::error!(err);
												}
											}
										}
										Err(err) => {
											seed::error!(err);
										}
									}
								}
							} else if let Some(temp_next) = str::strip_prefix(
								&key,
								&format!("{}record_", crate::storage::STORAGE_PREFIX),
							) {
								if let Ok(etag_found) = storage.get(&format!(
									"{}etag_record_{temp_next}",
									crate::storage::STORAGE_PREFIX
								)) {
									if etag_found.is_none() {
										match model.database_remote.as_ref().unwrap().put(
											&pontus_onyx::item::ItemPath::from(
												format!(
													"/triton_windstorm/records/{temp_next}.json"
												)
												.as_str(),
											),
											&pontus_onyx::item::Item::Document {
												etag: "*".into(),
												content: Some(value.as_bytes().to_vec()),
												content_type: "application/json".into(),
												last_modified: Some(time::OffsetDateTime::parse(
													&chrono::Utc::now().to_rfc2822(),
													&time::format_description::well_known::Rfc2822,
												).unwrap()),
											},
										) {
											Ok(promise) => {
												let temp_next_for_async = String::from(temp_next);

												orders.skip().perform_cmd(async move {
													let resp = seed::JsFuture::from(promise)
														.await
														.unwrap();

													let doc = resp
														.into_serde::<pontus_onyx::item::Item>()
														.unwrap();

													if let pontus_onyx::item::Item::Document {
														etag,
														last_modified,
														..
													} = doc
													{
														let storage =
															seed::prelude::web_sys::window()
																.unwrap()
																.local_storage()
																.unwrap()
																.unwrap();

														storage
															.set_item(
																&format!(
																	"{}etag_record_{temp_next_for_async}",
																	crate::storage::STORAGE_PREFIX
																),
																&format!("{etag}"),
															)
															.ok();

														if let Some(last_modified) = last_modified {
															if let Ok(last_modified) = last_modified.format(&time::format_description::well_known::Rfc3339) {
																storage
																	.set_item(
																		&format!(
																			"{}last_modified_record_{temp_next_for_async}",
																			crate::storage::STORAGE_PREFIX
																		),
																		&last_modified,
																	)
																	.ok();
															}
														} else {
															storage
																.remove_item(&format!(
																	"{}last_modified_record_{temp_next_for_async}",
																	crate::storage::STORAGE_PREFIX
																))
																.ok();
														}
													}
												});
											}
											Err(err) => {
												seed::error!(err);

												storage
													.remove_item(&format!(
														"{}etag_record_{temp_next}",
														crate::storage::STORAGE_PREFIX
													))
													.ok();

												storage
													.remove_item(&format!(
														"{}last_modified_record_{temp_next}",
														crate::storage::STORAGE_PREFIX
													))
													.ok();
											}
										}
									}
								}
							}
						}
					}

					// fetch subjects from remoteStorage and save it in localStorage if they are missing.
					match model.database_remote.as_ref().unwrap().get(
						&pontus_onyx::item::ItemPath::from("/triton_windstorm/subjects/"),
						None,
					) {
						Ok(promise) => {
							orders.skip().perform_cmd(async {
								let mut fetch_orders = vec![];

								let storage = seed::prelude::web_sys::window()
									.unwrap()
									.local_storage()
									.unwrap()
									.unwrap();

								let resp = seed::JsFuture::from(promise).await;

								match resp {
									Ok(resp) => {
										let doc =
											resp.into_serde::<pontus_onyx::item::Item>().unwrap();

										if let pontus_onyx::item::Item::Folder {
											etag: _,
											content: Some(children),
										} = doc
										{
											for (name, item) in children {
												if let Some(name) = name.strip_suffix(".json") {
													match storage.get(&format!(
														"{}subject_{name}",
														crate::storage::STORAGE_PREFIX
													)) {
														Ok(Some(_)) => {
															match storage.get(&format!(
																"{}etag_subject_{name}",
																crate::storage::STORAGE_PREFIX
															)) {
																Ok(Some(local_etag)) => {
																	if &pontus_onyx::item::Etag::from(
																		local_etag,
																	) != item.get_etag()
																	{
																		fetch_orders.push(crate::messages::Message::Database(
																			Message::FetchAndSaveSubjectFromRemote(String::from(name))
																		));
																	}
																}
																Ok(None) => {
																	// nothing to do here, it should be already done by previous block when save localStorage to remoteStorage.
																}
																Err(err) => seed::error!(err),
															}
														}
														Ok(None) => {
															fetch_orders
																.push(crate::messages::Message::Database(
																Message::FetchAndSaveSubjectFromRemote(
																	String::from(name),
																),
															));
														}
														Err(err) => seed::error!(err),
													}
												}
											}
										}

										crate::messages::Message::BatchMessages(fetch_orders)
									}
									Err(err) => {
										seed::error!(err);

										crate::messages::Message::None
									}
								}
							});
						}
						Err(err) => seed::error!(err),
					}

					// fetch records from remoteStorage and save it in localStorage if they are missing.
					match model.database_remote.as_ref().unwrap().get(
						&pontus_onyx::item::ItemPath::from("/triton_windstorm/records/"),
						None,
					) {
						Ok(promise) => {
							orders.skip().perform_cmd(async {
								let mut fetch_orders = vec![];

								let storage = seed::prelude::web_sys::window()
									.unwrap()
									.local_storage()
									.unwrap()
									.unwrap();

								let resp = seed::JsFuture::from(promise).await;

								match resp {
									Ok(resp) => {
										let doc =
											resp.into_serde::<pontus_onyx::item::Item>().unwrap();

										if let pontus_onyx::item::Item::Folder {
											etag: _,
											content: Some(children),
										} = doc
										{
											let name_regex =
												regex::Regex::new(RECORD_NAME_DATE).unwrap();
											for (name, item) in children {
												if let Some(name) = name.strip_suffix(".json") {
													if name_regex.is_match(name) {
														match storage.get(&format!(
															"{}record_{name}",
															crate::storage::STORAGE_PREFIX
														)) {
															Ok(Some(_)) => {
																match storage.get(&format!(
																	"{}etag_record_{name}",
																	crate::storage::STORAGE_PREFIX
																)) {
																	Ok(Some(local_etag)) => {
																		if &pontus_onyx::item::Etag::from(
																			local_etag,
																		) != item.get_etag()
																		{
																			fetch_orders.push(crate::messages::Message::Database(
																				Message::FetchAndSaveRecordFromRemote(String::from(name))
																			));
																		}
																	}
																	Ok(None) => {
																		// nothing to do here, it should be already done by previous block when save localStorage to remoteStorage.
																	}
																	Err(err) => seed::error!(err),
																}
															}
															Ok(None) => {
																fetch_orders
																	.push(crate::messages::Message::Database(
																	Message::FetchAndSaveRecordFromRemote(
																		String::from(name),
																	),
																));
															}
															Err(err) => seed::error!(err),
														}
													}
												}
											}
										}

										crate::messages::Message::BatchMessages(fetch_orders)
									}
									Err(err) => {
										seed::error!(err);

										crate::messages::Message::None
									}
								}
							});
						}
						Err(err) => seed::error!(err),
					}
				}
			}
		}
		Message::FetchAndSaveRecordFromRemote(record_id) => {
			let storage = seed::prelude::web_sys::window()
				.unwrap()
				.local_storage()
				.unwrap()
				.unwrap();

			if let Ok(Some(allowed_save)) =
				storage.get(&format!("{}allowed_save", crate::storage::STORAGE_PREFIX))
			{
				if allowed_save == "true" {
					if let Ok(promise) = model.database_remote.as_ref().unwrap().get(
						&pontus_onyx::item::ItemPath::from(
							format!("/triton_windstorm/records/{record_id}.json").as_str(),
						),
						None,
					) {
						orders.skip().perform_cmd(async move {
							let storage = seed::prelude::web_sys::window()
								.unwrap()
								.local_storage()
								.unwrap()
								.unwrap();

							if let Ok(resp) = seed::JsFuture::from(promise).await {
								let doc = resp.into_serde::<pontus_onyx::item::Item>().unwrap();

								if let pontus_onyx::item::Item::Document {
									content,
									etag,
									last_modified,
									..
								} = doc
								{
									if let Some(content) = content {
										let content = String::from_utf8(content);

										if let Ok(content) = content {
											storage
												.set_item(
													&format!(
														"{}record_{record_id}",
														crate::storage::STORAGE_PREFIX
													),
													&content,
												)
												.ok();
											storage
												.set_item(
													&format!(
														"{}etag_record_{record_id}",
														crate::storage::STORAGE_PREFIX
													),
													&etag.to_string(),
												)
												.ok();

											if let Some(last_modified) = last_modified {
												if let Ok(last_modified) = last_modified.format(
													&time::format_description::well_known::Rfc3339,
												) {
													storage
														.set_item(
															&format!(
																"{}last_modified_record_{record_id}",
																crate::storage::STORAGE_PREFIX
															),
															&last_modified,
														)
														.ok();
												}
											} else {
												storage
													.remove_item(&format!(
														"{}last_modified_record_{record_id}",
														crate::storage::STORAGE_PREFIX
													))
													.ok();
											}
										}
									}
								} else {
									panic!();
								};
							}
						});
					}
				}
			}
		}
		Message::FetchAndSaveSubjectFromRemote(subject_id) => {
			let storage = seed::prelude::web_sys::window()
				.unwrap()
				.local_storage()
				.unwrap()
				.unwrap();

			if let Ok(Some(allowed_save)) =
				storage.get(&format!("{}allowed_save", crate::storage::STORAGE_PREFIX))
			{
				if allowed_save == "true" {
					if let Ok(promise) = model.database_remote.as_ref().unwrap().get(
						&pontus_onyx::item::ItemPath::from(
							format!("/triton_windstorm/subjects/{subject_id}.json").as_str(),
						),
						None,
					) {
						orders.skip().perform_cmd(async move {
							let storage = seed::prelude::web_sys::window()
								.unwrap()
								.local_storage()
								.unwrap()
								.unwrap();

							if let Ok(resp) = seed::JsFuture::from(promise).await {
								let doc = resp.into_serde::<pontus_onyx::item::Item>().unwrap();

								if let pontus_onyx::item::Item::Document {
									content,
									etag,
									last_modified,
									..
								} = doc
								{
									if let Some(content) = content {
										let content = String::from_utf8(content);

										if let Ok(content) = content {
											storage
												.set_item(
													&format!(
														"{}subject_{subject_id}",
														crate::storage::STORAGE_PREFIX
													),
													&content,
												)
												.ok();
											storage
												.set_item(
													&format!(
														"{}etag_subject_{subject_id}",
														crate::storage::STORAGE_PREFIX
													),
													&etag.to_string(),
												)
												.ok();

											if let Some(last_modified) = last_modified {
												if let Ok(last_modified) = last_modified.format(
													&time::format_description::well_known::Rfc3339,
												) {
													storage
														.set_item(
															&format!(
																"{}last_modified_subject_{subject_id}",
																crate::storage::STORAGE_PREFIX
															),
															&last_modified,
														)
														.ok();
												}
											} else {
												storage
													.remove_item(&format!(
														"{}last_modified_subject_{subject_id}",
														crate::storage::STORAGE_PREFIX
													))
													.ok();
											}
										}
									}
								} else {
									panic!();
								};
							}
						});
					}
				}
			}
		}
	}
}
