pub enum Message {
	SetPretty(bool),

	SetPendingImport(String),
	DoImport(bool),
}

pub fn update(
	message: Message,
	model: &mut crate::model::Model,
	orders: &mut impl seed::prelude::Orders<crate::messages::Message>,
) {
	match message {
		Message::SetPretty(val) => model.pretty_export = val,
		Message::SetPendingImport(value) => model.pending_import = value,
		Message::DoImport(replace) => {
			if replace == true {
				model.subjects = std::collections::HashMap::new();
				model.records = std::collections::HashMap::new();
			}

			if let Ok(data) = serde_json::from_str(&model.pending_import)
				as serde_json::Result<crate::model::Export>
			{
				if let Some(subjects) = data.subjects {
					for subject in subjects {
						if !model.subjects.contains_key(&subject.id) {
							model.subjects.insert(subject.id.clone(), subject.clone());
							orders.send_msg(crate::messages::Message::Settings(
								crate::messages::settings::Message::SetSubjectName {
									id: subject.id.clone(),
									name: subject.name,
								},
							));
							orders.send_msg(crate::messages::Message::Settings(
								crate::messages::settings::Message::SetSubjectMax {
									id: subject.id,
									max: format!("{}", subject.max),
								},
							));
						} else {
							seed::log!(format!("subject {} already exists", subject.id));
						}
					}
				}

				if let Some(records) = data.records {
					for record in records {
						let day = format!("{}", record.date.format("%Y-%m-%d"));
						if !model.records.contains_key(&day) {
							orders.send_msg(crate::messages::Message::SaveStorage {
								key: format!("record_{}", day),
								value: serde_json::to_string(&record).unwrap(),
							});

							model.records.insert(day, record.clone());
						} else {
							seed::log!(format!("record {} already exists", day));
						}
					}
				}

				orders.send_msg(crate::messages::Message::Index(
					crate::messages::index::Message::SetDateToday,
				));
			}
		}
	}
}
