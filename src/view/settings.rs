use seed::{prelude::*, *};

pub fn view(model: &crate::model::Model) -> Node<crate::message::Message> {
	let mut temp = model.pending_rate.subjects.clone();
	temp.append(&mut vec![crate::model::Subject {
		id: format!("{}", uuid::Uuid::new_v4()),
		name: String::from(""),
		value: None,
		max: 5,
		observations: None,
	}]);
	let settings_subjects_fields = temp.iter().map(|subject| {
		let id_subject = subject.id.clone();
		let id_max = subject.id.clone();

		return nodes![
			label![
				format!("{} : ", crate::locale::get_simple(&model.locale, "name")),
				input![
					attrs![
						At::Type => "text",
						At::Value => subject.name,
					],
					ev(Ev::Blur, move |event| {
						let target = wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlInputElement>(
							event.target().unwrap(),
						)
						.unwrap();
						crate::message::Message::SetSubjectName {
							id: id_subject,
							name: target.value(),
						}
					}),
				],
			],
			raw!(" "),
			label![
				format!("{} : ", crate::locale::get_simple(&model.locale, "max")),
				input![
					attrs![
						At::Type => "number",
						At::Value => subject.max,
					],
					ev(Ev::Change, move |event| {
						let target = wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlInputElement>(
							event.target().unwrap(),
						)
						.unwrap();
						crate::message::Message::SetSubjectMax {
							id: id_max,
							max: target.value(),
						}
					}),
				],
			],
			br![],
		];
	});

	return article![
		h2![crate::locale::get_simple(&model.locale, "settings")],
		p![
			raw!(&format!(
				"{} (locale) : ",
				crate::locale::get_simple(&model.locale, "locale"),
			)),
			button![
				"EN",
				ev(Ev::Click, |_| crate::message::Message::SetLocale {
					locale: String::from("en-US")
				}),
			],
			raw!(" "),
			button![
				"FR",
				ev(Ev::Click, |_| crate::message::Message::SetLocale {
					locale: String::from("fr-FR")
				}),
			],
		],
		p![format!(
			"{} :",
			crate::locale::get_simple(&model.locale, "subjects")
		)],
		settings_subjects_fields,
		p![input![attrs![
			At::Type => "submit",
			At::Value => crate::locale::get_simple(&model.locale, "save"),
		],],],
		p!(format!(
			"{} : {}",
			crate::locale::get_simple(&model.locale, "app-version"),
			env!("CARGO_PKG_VERSION")
		)),
	];
}
