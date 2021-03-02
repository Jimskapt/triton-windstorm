use seed::{prelude::*, *};

pub fn view(model: &crate::model::Model) -> Node<crate::message::Message> {
	let notation_subjects = model.pending_rate.subjects.iter().map(|subject| {
		let id_value = subject.id.clone();
		let id_observation = subject.id.clone();
		let id_delete = subject.id.clone();

		if subject.name.trim() != "" {
			return tr![
				td![&subject.name],
				td![input![
					attrs![
						At::Type => "range",
						At::Min => 0,
						At::Step => 0.1,
						At::Max => subject.max,
						At::Value => subject.value.unwrap_or(subject.max),
					],
					input_ev(Ev::Input, move |value| {
						crate::message::Message::SetSubjectValue {
							id: id_value,
							value: Some(value),
						}
					}),
				]],
				td![format!(
					"{:04.1} / {:02} ",
					subject.value.unwrap_or(subject.max),
					subject.max
				)],
				td![input![
					attrs![
						At::Type => "text",
						At::Placeholder => crate::locale::get_simple(&model.locale, "observation"),
						At::Value => subject.observations.clone().unwrap_or(String::new()),
					],
					input_ev(Ev::Input, move |value| {
						crate::message::Message::SetSubjectObservation {
							id: id_observation,
							observation: Some(value),
						}
					}),
				],],
				td![IF![
					subject.value.is_some() || subject.observations.is_some() => button![
						attrs![
							At::Type => "date",
						],
						"X",
						ev(Ev::Click, |_| {
							crate::message::Message::RemoveSingleRate{id: id_delete}
						}),
					]
				],],
			];
		} else {
			return tr![];
		}
	});

	let date_for_previous = model.pending_rate.date.clone();
	let date_for_next = model.pending_rate.date.clone();

	return article![
		h2![crate::locale::get_simple(&model.locale, "rate-a-day")],
		p![
			button![
				attrs![
					At::Type => "date",
				],
				"<",
				ev(Ev::Click, move |_| {
					crate::message::Message::SetRateDay {
						day: format!(
							"{}",
							(date_for_previous - chrono::Duration::days(1)).format("%Y-%m-%d")
						),
					}
				}),
			],
			raw!(" "),
			label![
				format!("{} : ", crate::locale::get_simple(&model.locale, "day")),
				input![
					attrs![
						At::Type => "date",
						At::Id => "notation_date",
						At::Value => model.pending_rate.date,
					],
					ev(Ev::Change, |event| {
						let target = wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlInputElement>(
							event.target().unwrap(),
						)
						.unwrap();

						crate::message::Message::SetRateDay {
							day: target.value(),
						}
					}),
				],
			],
			raw!(" "),
			button![
				attrs![
					At::Type => "date",
				],
				">",
				ev(Ev::Click, move |_| {
					crate::message::Message::SetRateDay {
						day: format!(
							"{}",
							(date_for_next + chrono::Duration::days(1)).format("%Y-%m-%d")
						),
					}
				}),
			],
			br![],
			button![
				attrs![
					At::Type => "button",
				],
				ev(Ev::Click, |_| crate::message::Message::SetDateToday),
				crate::locale::get_simple(&model.locale, "today"),
			],
		],
		table![notation_subjects,],
		input![
			attrs![
				At::Type => "submit",
				At::Value => crate::locale::get_simple(&model.locale, "save"),
			],
			ev(Ev::Click, |_| crate::message::Message::SaveRate),
		],
		raw!(" "),
		input![
			attrs![
				At::Type => "reset",
				At::Value => crate::locale::get_simple(&model.locale, "reset"),
			],
			ev(Ev::Click, |_| crate::message::Message::ResetSubjects),
		],
	];
}
