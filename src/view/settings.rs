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
		steps: 1.0,
	});
	let settings_subjects_fields = temp.iter().map(|subject| {
		let id_subject = subject.id.clone();
		let id_max = subject.id.clone();
		let id_steps = subject.id.clone();

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
			label![
				attrs![
					At::For => format!("steps-{}", &subject.id),
				],
				crate::locale::get_simple(&model.locale, "steps"),
			],
			input![
				attrs![
					At::Type => "number",
					At::Value => subject.steps,
					At::Id => format!("steps-{}", &subject.id),
				],
				ev(Ev::Change, move |event| {
					let target = wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlInputElement>(
						event.target().unwrap(),
					)
					.unwrap();
					crate::messages::Message::Settings(
						crate::messages::settings::Message::SetSubjectSteps {
							id: id_steps,
							steps: target.value(),
						},
					)
				}),
			],
		];
	});

	return div![
		C!["settings_page"],
		h2![crate::locale::get_simple(&model.locale, "settings")],
		h3![crate::locale::get_simple(&model.locale, "subjects")],
		article![settings_subjects_fields,],
		h3![crate::locale::get_simple(&model.locale, "theme")],
		article![
			button![
				attrs![
					At::Type => "button",
				],
				format!(
					"\u{1f4a1} {}",
					crate::locale::get_simple(&model.locale, "light-theme")
				),
				ev(Ev::Click, |_| crate::messages::Message::Settings(
					crate::messages::settings::Message::SetDarkTheme { value: false }
				)),
			],
			raw!(" "),
			button![
				attrs![
					At::Type => "button",
				],
				format!(
					"\u{1f319} {}",
					crate::locale::get_simple(&model.locale, "dark-theme")
				),
				ev(Ev::Click, |_| crate::messages::Message::Settings(
					crate::messages::settings::Message::SetDarkTheme { value: true }
				)),
			],
		],
		h3![format!(
			"{} (locale)",
			crate::locale::get_simple(&model.locale, "locale"),
		)],
		article![
			button![
				attrs![
					At::Type => "button",
				],
				"\u{1f1fa}\u{1f1f8} \u{1f1ec}\u{1f1e7} EN",
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
				"\u{1f1eb}\u{1f1f7} FR",
				ev(Ev::Click, |_| crate::messages::Message::Settings(
					crate::messages::settings::Message::SetLocale {
						locale: String::from("fr-FR")
					}
				)),
			],
		],
		h3![crate::locale::get_simple(&model.locale, "debug")],
		article![button![
			attrs![
				At::Type => "button",
			],
			format!(
				"\u{1f504} {}",
				crate::locale::get_simple(&model.locale, "force-refresh")
			),
			ev(Ev::Click, |_| crate::messages::Message::Settings(
				crate::messages::settings::Message::ForceRefresh
			)),
		],],
	];
}
