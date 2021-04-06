pub enum Message {
	SetRateDay {
		day: String,
	},
	SetDateToday,

	SetSubjectValue {
		id: String,
		value: Option<String>,
	},
	SetSubjectObservation {
		id: String,
		observation: Option<String>,
	},

	SaveRate,

	ResetSubject {
		id: String,
	},
	ResetSubjects,
}

pub fn update(
	message: Message,
	model: &mut crate::model::Model,
	orders: &mut impl seed::prelude::Orders<crate::messages::Message>,
) {
	let storage = seed::prelude::web_sys::window()
		.unwrap()
		.local_storage()
		.unwrap()
		.unwrap();

	match message {
		Message::SetRateDay { day } => {
			let date = chrono::NaiveDate::parse_from_str(&day, "%Y-%m-%d").unwrap_or_else(|_| {
				orders.send_msg(crate::messages::Message::Index(Message::SetDateToday));
				let today = chrono::offset::Local::today();
				chrono::NaiveDate::from_ymd(
					chrono::Datelike::year(&today),
					chrono::Datelike::month(&today),
					chrono::Datelike::day(&today),
				)
			});
			let default_rate = crate::model::Rate {
				date,
				subjects: model.subjects.values().cloned().collect(),
			};

			let mut selected_rate = if let Some(result) = model.records.get(&day) {
				result.clone()
			} else {
				match storage
					.get_item(&format!("{}record_{}", crate::storage::STORAGE_PREFIX, day))
					.unwrap()
				{
					Some(res) => match serde_json::from_str::<crate::model::Rate>(&res) {
						Ok(res) => {
							model.records.insert(day, res.clone());
							res
						}
						Err(_) => default_rate,
					},
					None => default_rate,
				}
			};

			for subject in model.subjects.values() {
				match selected_rate
					.subjects
					.iter_mut()
					.find(|e| e.id == subject.id)
				{
					Some(data_subject) => {
						data_subject.name = subject.name.clone();
						if let Some(val) = data_subject.value {
							data_subject.value = Some((val / data_subject.max) * subject.max);
						}
						data_subject.max = subject.max;
					}
					None => {
						selected_rate.subjects.push(subject.clone());
					}
				}
			}

			if selected_rate.subjects.is_empty() {
				selected_rate.subjects.push(crate::model::Subject {
					id: format!("{}", uuid::Uuid::new_v4()),
					name: String::from("mood"),
					max: 5.0,
					value: None,
					observations: None,
				});
			}

			selected_rate
				.subjects
				.sort_by_key(|subject| subject.name.clone());

			model.pending_rate = selected_rate;
		}
		Message::SetDateToday => {
			orders.send_msg(crate::messages::Message::Index(Message::SetRateDay {
				day: format!("{}", chrono::offset::Local::today().format("%Y-%m-%d")),
			}));
		}

		Message::SetSubjectValue { id, value } => {
			if let Some(subject) = model
				.pending_rate
				.subjects
				.iter_mut()
				.find(|subject| subject.id == id)
			{
				subject.value = match value {
					Some(value) => Some(std::str::FromStr::from_str(&value).unwrap_or(0.0) as f64),
					None => None,
				};
			}
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

		Message::SaveRate => {
			let key = format!("{}", model.pending_rate.date.format("%Y-%m-%d"));

			orders.send_msg(crate::messages::Message::SaveStorage {
				key: format!("record_{}", key),
				value: serde_json::to_string(&model.pending_rate).unwrap(),
			});

			model.records.insert(key, model.pending_rate.clone());
		}

		Message::ResetSubject { id } => {
			orders.send_msg(crate::messages::Message::Index(Message::SetSubjectValue {
				id: id.clone(),
				value: None,
			}));
			orders.send_msg(crate::messages::Message::Index(
				Message::SetSubjectObservation {
					id,
					observation: None,
				},
			));
		}
		Message::ResetSubjects => {
			for subject in &model.pending_rate.subjects {
				orders.send_msg(crate::messages::Message::Index(Message::ResetSubject {
					id: subject.id.clone(),
				}));
			}
		}
	}
}
