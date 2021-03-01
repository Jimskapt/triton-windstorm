pub enum Message {
	SetLocale {
		locale: String,
	},

	SaveStorage {
		key: String,
		value: String,
	},
	AllowStorage,
	DismissStorageWarning,

	GoToPanel {
		panel: crate::model::AppPanel,
	},

	SetDateToday,

	SetSubjectName {
		id: String,
		name: String,
	},
	SetSubjectValue {
		id: String,
		value: Option<String>,
	},
	SetSubjectMax {
		id: String,
		max: String,
	},
	SetSubjectObservation {
		id: String,
		observation: Option<String>,
	},
	RemoveSingleRate {
		id: String,
	},
	ResetSubject {
		id: String,
	},
	ResetSubjects,

	SetRateDay {
		day: String,
	},
	SaveRate,
}

pub fn update(
	message: crate::message::Message,
	model: &mut crate::model::Model,
	orders: &mut impl seed::prelude::Orders<crate::message::Message>,
) {
	let storage = seed::prelude::web_sys::window()
		.unwrap()
		.local_storage()
		.unwrap()
		.unwrap();

	match message {
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
		Message::SetLocale { locale } => {
			model.locale = crate::locale::get_bundle(&locale);
			orders.send_msg(Message::SaveStorage {
				key: String::from("locale"),
				value: locale,
			});
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
		Message::SetSubjectName { id, name } => {
			match model
				.pending_rate
				.subjects
				.iter_mut()
				.find(|subject| subject.id == id)
			{
				Some(subject) => {
					subject.name = name;

					orders.send_msg(Message::SaveStorage {
						key: format!("subject_{}_name", subject.id.clone()),
						value: subject.name.clone(),
					});
				}
				None => {
					let temp = crate::model::Subject {
						id: format!("{}", uuid::Uuid::new_v4()),
						name,
						value: None,
						max: 5,
						observations: None,
					};

					orders.send_msg(Message::SaveStorage {
						key: format!("subject_{}_name", temp.id.clone()),
						value: temp.name.clone(),
					});

					model.pending_rate.subjects.push(temp);
				}
			}
		}
		Message::SetSubjectValue { id, value } => {
			if let Some(subject) = model
				.pending_rate
				.subjects
				.iter_mut()
				.find(|subject| subject.id == id)
			{
				subject.value = match value {
					Some(value) => Some(std::str::FromStr::from_str(&value).unwrap_or(0) as usize),
					None => None,
				};
			}
		}
		Message::SetSubjectMax { id, max } => {
			let value: Result<isize, _> = max.parse();

			if let Ok(value) = value {
				if value >= 1 && value <= isize::max_value() {
					let max = std::convert::TryInto::try_into(value).unwrap();

					match model
						.pending_rate
						.subjects
						.iter_mut()
						.find(|subject| subject.id == id)
					{
						Some(subject) => {
							subject.max = max;

							orders.send_msg(Message::SaveStorage {
								key: format!("subject_{}_max", subject.id.clone()),
								value: format!("{}", subject.max.clone()),
							});

							if let Some(val) = subject.value {
								if val > subject.max {
									orders.send_msg(Message::SetSubjectValue {
										id: subject.id.clone(),
										value: Some(format!("{}", subject.max)),
									});
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

							orders.send_msg(Message::SaveStorage {
								key: format!("subject_{}_max", temp.id.clone()),
								value: format!("{}", temp.max),
							});
						}
					}
				}
			}
		}
		Message::GoToPanel { panel } => {
			model.panel = panel;
		}
		Message::SetDateToday => {
			orders.send_msg(Message::SetRateDay {
				day: format!("{}", chrono::offset::Local::today().format("%Y-%m-%d")),
			});
		}
		Message::SetSubjectObservation { id, observation } => {
			if let Some(subject) = model
				.pending_rate
				.subjects
				.iter_mut()
				.find(|subject| subject.id == id)
			{
				subject.observations = observation;
			}
		}
		Message::ResetSubject { id } => {
			orders.send_msg(Message::SetSubjectValue {
				id: id.clone(),
				value: None,
			});
			orders.send_msg(Message::SetSubjectObservation {
				id,
				observation: None,
			});
		}
		Message::ResetSubjects => {
			for subject in &model.pending_rate.subjects {
				orders.send_msg(Message::ResetSubject {
					id: subject.id.clone(),
				});
			}
		}
		Message::SetRateDay { day } => {
			let mut subjects: Vec<crate::model::Subject> = vec![];
			if let Ok(length) = storage.length() {
				for i in 0..length {
					let key = storage.key(i).unwrap().unwrap();
					let value = storage.get(&key).unwrap().unwrap();

					if let Some(temp_next) = str::strip_prefix(
						&key,
						&format!("{}subject_", crate::storage::STORAGE_PREFIX),
					) {
						if let Some(id) = temp_next.strip_suffix("_name") {
							match subjects.iter_mut().find(|e| e.id == id) {
								Some(subject) => {
									subject.name = value.clone();
								}
								None => {
									subjects.push(crate::model::Subject {
										id: String::from(id),
										name: value.clone(),
										max: 5,
										value: None,
										observations: None,
									});
								}
							}
						}

						if let Some(id) = temp_next.strip_suffix("_max") {
							let value = value.parse().unwrap();

							match subjects.iter_mut().find(|e| e.id == id) {
								Some(subject) => {
									subject.max = value;
								}
								None => {
									subjects.push(crate::model::Subject {
										id: String::from(id),
										name: String::new(),
										max: value,
										value: None,
										observations: None,
									});
								}
							}
						}
					}
				}
			}

			if subjects.is_empty() {
				subjects.push(crate::model::Subject {
					id: format!("{}", uuid::Uuid::new_v4()),
					name: String::from("mood"),
					max: 5,
					value: None,
					observations: None,
				});
			}

			model.pending_rate = match storage
				.get_item(&format!("{}record_{}", crate::storage::STORAGE_PREFIX, day))
				.unwrap()
			{
				Some(res) => {
					serde_json::from_str(&res).unwrap() // TODO : check errors ?
					                // TODO : merge new subjects ?
				}
				None => {
					let date =
						chrono::NaiveDate::parse_from_str(&day, "%Y-%m-%d").unwrap_or_else(|_| {
							orders.send_msg(Message::SetDateToday);
							let today = chrono::offset::Local::today();
							chrono::NaiveDate::from_ymd(
								chrono::Datelike::year(&today),
								chrono::Datelike::month(&today),
								chrono::Datelike::day(&today),
							)
						});
					crate::model::Rate { date, subjects }
				}
			};

			model
				.pending_rate
				.subjects
				.sort_by_key(|subject| subject.id.clone());
		}
		Message::SaveRate => {
			orders.send_msg(Message::SaveStorage {
				key: format!("record_{}", &model.pending_rate.date.format("%Y-%m-%d")),
				value: serde_json::to_string(&model.pending_rate).unwrap(),
			});
		}
		Message::RemoveSingleRate { id } => {
			orders.send_msg(Message::SetSubjectValue {
				id: id.clone(),
				value: None,
			});
			orders.send_msg(Message::SetSubjectObservation {
				id,
				observation: None,
			});
		}
	}
}
