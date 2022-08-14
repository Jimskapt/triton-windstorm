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
		source: crate::model::SubjectSource::User,
	});
	let settings_subjects_fields = temp.iter().map(|subject| {
		let id_subject = subject.id.clone();
		let id_max = subject.id.clone();
		let id_steps = subject.id.clone();

		return div![
			attrs![
				At::Class => "single_subject",
			],
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
					At::Class => "tw-w-full",
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
					At::Class => "tw-w-full",
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
					At::Class => "tw-w-full",
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

	let connection_status = model
		.database_remote
		.as_ref()
		.map(|remote| remote.is_connected())
		.unwrap_or(false);

	return div![
		C!["settings_page"],
		h2![crate::locale::get_simple(&model.locale, "settings")],
		h3![crate::locale::get_simple(&model.locale, "subjects")],
		article![
			attrs![
				At::Id => "all_subjects",
			],
			settings_subjects_fields,
		],
		h3![crate::locale::get_simple(&model.locale, "database")],
		article![
			label![
				attrs![
					At::Class => "tw-col-span-12",
				],
				crate::locale::get_simple(&model.locale, "current-status"),
			],
			p![
				attrs![
					At::Class => "tw-col-span-12",
				],
				strong![
					style![
						St::Color => if connection_status {
							"rgb(74 222 128)"
						} else {
							"rgb(253 186 116)"
						}
					],
					if connection_status {
						format!(
							"\u{2611} {}",
							crate::locale::get_simple(&model.locale, "connected")
						)
					} else {
						format!(
							"\u{1f50c} {}",
							crate::locale::get_simple(&model.locale, "disconnected")
						)
					}
				]
			],
			label![
				attrs![
					At::For => "rs_account",
					At::Class => "tw-col-span-12",
				],
				crate::locale::get_simple(&model.locale, "account"),
			],
			input![
				attrs![
					At::Type => "text",
					At::Id => "rs_account",
					At::Placeholder => format!("{}@{}(:{})", crate::locale::get_simple(&model.locale, "username"), crate::locale::get_simple(&model.locale, "domain"), crate::locale::get_simple(&model.locale, "port")),
					At::Value => model.database_account_string,
					At::Class => "tw-col-span-12",
					At::Disabled => connection_status.as_at_value(),
				],
				ev(Ev::Input, |event| crate::messages::Message::Database(
					crate::messages::database::Message::SetDataAccountString(event)
				))
			],
			button![
				attrs![
					At::Type => "button",
					At::Class => "tw-col-span-12",
				],
				if connection_status {
					format!(
						"\u{1f50c} {}",
						crate::locale::get_simple(&model.locale, "disconnect")
					)
				} else {
					format!(
						"\u{26a1} {}",
						crate::locale::get_simple(&model.locale, "connect")
					)
				},
				if connection_status {
					ev(Ev::Click, |_| {
						crate::messages::Message::Database(
							crate::messages::database::Message::DropRemote,
						)
					})
				} else {
					ev(Ev::Click, |_| {
						crate::messages::Message::Database(
							crate::messages::database::Message::BuildRemote(Some(Box::new(
								crate::messages::Message::Database(
									crate::messages::database::Message::SyncDatabase,
								),
							))),
						)
					})
				}
			],
		],
		h3![crate::locale::get_simple(&model.locale, "theme")],
		article![
			button![
				attrs![
					At::Type => "button",
					At::Class => "tw-col-span-12 sm:tw-col-span-6",
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
					At::Class => "tw-col-span-12 sm:tw-col-span-6",
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
					At::Class => "tw-col-span-12 sm:tw-col-span-6",
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
					At::Class => "tw-col-span-12 sm:tw-col-span-6",
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
				At::Class => "tw-col-span-12"
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
