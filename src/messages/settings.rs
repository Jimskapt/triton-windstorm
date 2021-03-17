pub enum Message {
	SetLocale { locale: String },

	SetDarkTheme { value: bool },

	SetSubjectName { id: String, name: String },
	SetSubjectMax { id: String, max: String },

	SubjectsCleanup,
}

pub fn update(
	message: Message,
	model: &mut crate::model::Model,
	orders: &mut impl seed::prelude::Orders<crate::messages::Message>,
) {
	let _storage = seed::prelude::web_sys::window()
		.unwrap()
		.local_storage()
		.unwrap()
		.unwrap();

	match message {
		Message::SetLocale { locale } => {
			model.locale = crate::locale::get_bundle(&locale);
			orders.send_msg(crate::messages::Message::SaveStorage {
				key: String::from("locale"),
				value: locale,
			});
		}

		Message::SetDarkTheme { value } => {
			model.dark_theme = value;

			orders.send_msg(crate::messages::Message::SaveStorage {
				key: String::from("dark_theme"),
				value: String::from(match value {
					true => "true",
					false => "false",
				}),
			});

			let html_classes = seed::prelude::wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(
				seed::prelude::web_sys::window()
					.unwrap()
					.document()
					.unwrap()
					.document_element()
					.unwrap(),
			)
			.unwrap()
			.class_list();
			if value {
				html_classes.add_1("tw-dark").unwrap();
			} else {
				html_classes.remove_1("tw-dark").unwrap();
			}
		}

		Message::SetSubjectName { id, name } => {
			match model.subjects.get_mut(&id) {
				Some(subject) => {
					(*subject).name = name;

					orders.send_msg(crate::messages::Message::SaveStorage {
						key: format!("subject_{}_name", subject.id.clone()),
						value: subject.name.clone(),
					});
				}
				None => {
					let temp = crate::model::Subject {
						id: format!("{}", uuid::Uuid::new_v4()),
						name,
						value: None,
						max: 5.0,
						observations: None,
					};

					orders.send_msg(crate::messages::Message::SaveStorage {
						key: format!("subject_{}_name", temp.id.clone()),
						value: temp.name.clone(),
					});

					model.subjects.insert(temp.id.clone(), temp);
				}
			}

			orders.send_msg(crate::messages::Message::Settings(
				crate::messages::settings::Message::SubjectsCleanup,
			));

			orders.send_msg(crate::messages::Message::Index(
				crate::messages::index::Message::SetRateDay {
					day: format!("{}", model.pending_rate.date.format("%Y-%m-%d")),
				},
			));
		}
		Message::SetSubjectMax { id, max } => {
			let value: Result<f64, _> = max.parse();

			if let Ok(value) = value {
				if value >= 1.0 && value <= f64::MAX {
					let max = std::convert::TryInto::try_into(value).unwrap();

					match model.subjects.get_mut(&id) {
						Some(subject) => {
							(*subject).max = max;

							orders.send_msg(crate::messages::Message::SaveStorage {
								key: format!("subject_{}_max", subject.id.clone()),
								value: format!("{}", subject.max.clone()),
							});

							if let Some(val) = subject.value {
								if val > subject.max {
									orders.send_msg(crate::messages::Message::Index(
										crate::messages::index::Message::SetSubjectValue {
											id: subject.id.clone(),
											value: Some(format!("{}", subject.max)),
										},
									));
								}
							}
						}
						None => {
							let temp = crate::model::Subject {
								id: format!("{}", uuid::Uuid::new_v4()),
								name: String::new(),
								value: None,
								max,
								observations: None,
							};

							model.pending_rate.subjects.push(temp.clone());

							orders.send_msg(crate::messages::Message::SaveStorage {
								key: format!("subject_{}_max", temp.id.clone()),
								value: format!("{}", temp.max),
							});
						}
					}
				}
			}

			orders.send_msg(crate::messages::Message::Index(
				crate::messages::index::Message::SetRateDay {
					day: format!("{}", model.pending_rate.date.format("%Y-%m-%d")),
				},
			));
		}
		Message::SubjectsCleanup => {
			model.subjects = model
				.subjects
				.iter()
				.filter(|(_, subject)| {
					let res = subject.name.trim() != "";

					if !res {
						orders.send_msg(crate::messages::Message::DeleteStorage(format!(
							"subject_{}_name",
							subject.id.clone()
						)));
						orders.send_msg(crate::messages::Message::DeleteStorage(format!(
							"subject_{}_max",
							subject.id.clone()
						)));
					}

					res
				})
				.map(|(k, v)| (k.clone(), v.clone()))
				.collect();
		}
	}
}
