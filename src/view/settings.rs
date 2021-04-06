use seed::{prelude::*, *};

pub fn view(model: &crate::model::Model) -> Node<crate::messages::Message> {
	let mut temp: Vec<crate::model::Subject> = model.subjects.values().cloned().collect();
	temp.sort_by_key(|subject| subject.name.clone());
	temp.push(crate::model::Subject {
		id: format!("{}", uuid::Uuid::new_v4()),
		name: String::from(""),
		value: None,
		max: 5.0,
		observations: None,
	});
	let settings_subjects_fields = temp.iter().map(|subject| {
		let id_subject = subject.id.clone();
		let id_max = subject.id.clone();

		return div![
			C!["subject"],
			label![
				attrs![
					At::For => format!("name-{}", &subject.id),
				],
				crate::locale::get_simple(&model.locale, "name"),
			],
			input![
				attrs![
					At::Type => "text",
					At::Value => subject.name,
					At::Id => format!("name-{}", &subject.id),
				],
				ev(Ev::Blur, move |event| {
					let target = wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlInputElement>(
						event.target().unwrap(),
					)
					.unwrap();

					crate::messages::Message::Settings(
						crate::messages::settings::Message::SetSubjectName {
							id: id_subject,
							name: target.value(),
						},
					)
				}),
			],
			label![
				attrs![
					At::For => format!("max-{}", &subject.id),
				],
				crate::locale::get_simple(&model.locale, "max"),
			],
			input![
				attrs![
					At::Type => "number",
					At::Value => subject.max,
					At::Id => format!("max-{}", &subject.id),
				],
				ev(Ev::Change, move |event| {
					let target = wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlInputElement>(
						event.target().unwrap(),
					)
					.unwrap();
					crate::messages::Message::Settings(
						crate::messages::settings::Message::SetSubjectMax {
							id: id_max,
							max: target.value(),
						},
					)
				}),
			],
		];
	});

	return div![
		C!["settings_page"],
		h2![crate::locale::get_simple(&model.locale, "settings")],
		h3![format!(
			"{} (locale)",
			crate::locale::get_simple(&model.locale, "locale"),
		)],
		article![
			button![
				attrs![
					At::Type => "button",
				],
				"EN",
				ev(Ev::Click, |_| crate::messages::Message::Settings(
					crate::messages::settings::Message::SetLocale {
						locale: String::from("en-US")
					}
				)),
			],
			raw!(" "),
			button![
				attrs![
					At::Type => "button",
				],
				"FR",
				ev(Ev::Click, |_| crate::messages::Message::Settings(
					crate::messages::settings::Message::SetLocale {
						locale: String::from("fr-FR")
					}
				)),
			],
		],
		h3![crate::locale::get_simple(&model.locale, "theme")],
		article![
			button![
				attrs![
					At::Type => "button",
				],
				crate::locale::get_simple(&model.locale, "light-theme"),
				ev(Ev::Click, |_| crate::messages::Message::Settings(
					crate::messages::settings::Message::SetDarkTheme { value: false }
				)),
			],
			raw!(" "),
			button![
				attrs![
					At::Type => "button",
				],
				crate::locale::get_simple(&model.locale, "dark-theme"),
				ev(Ev::Click, |_| crate::messages::Message::Settings(
					crate::messages::settings::Message::SetDarkTheme { value: true }
				)),
			],
		],
		h3![crate::locale::get_simple(&model.locale, "subjects")],
		article![settings_subjects_fields,],
	];
}
