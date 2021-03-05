use seed::{prelude::*, *};

pub fn view(model: &crate::model::Model) -> Node<crate::message::Message> {
	let notation_subjects = model.pending_rate.subjects.iter().map(|subject| {
		let id_value = subject.id.clone();
		let id_observation = subject.id.clone();

		if subject.name.trim() != "" {
			return div![
				C!["subject"],
				div![
					C!["rate"],
					label![
						attrs![
							At::For => format!("rate-{}", subject.id),
						],
						format!("{} :", &subject.name),
					],
					raw![&format!("{:04.1} ", subject.value.unwrap_or(subject.max))],
					input![
						attrs![
							At::Type => "range",
							At::Min => 0,
							At::Step => 0.1,
							At::Max => subject.max,
							At::Value => subject.value.unwrap_or(subject.max),
							At::Id => format!("rate-{}", subject.id),
						],
						input_ev(Ev::Input, move |value| {
							crate::message::Message::SetSubjectValue {
								id: id_value,
								value: Some(value),
							}
						}),
					],
					raw![&format!(" {}", subject.max)],
				],
				div![
					C!["observation"],
					textarea![
						attrs![
							At::Type => "text",
							At::Placeholder => crate::locale::get_simple(&model.locale, "observation"),
							At::Rows => 1,
							At::Value => subject.observations.clone().unwrap_or(String::new()),
						],
						input_ev(Ev::Input, move |value| {
							crate::message::Message::SetSubjectObservation {
								id: id_observation,
								observation: Some(value),
							}
						}),
					],
				],
				hr![],
			];
		} else {
			return tr![];
		}
	});

	let date_for_previous = model.pending_rate.date.clone();
	let date_for_next = model.pending_rate.date.clone();

	return div![
		C!["index_page",],
		h2![crate::locale::get_simple(&model.locale, "rate-a-day")],
		article![
			C!["date_selection"],
			button![
				attrs![
					At::Type => "button",
					At::Id => "previous",
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
			label![
				attrs![
					At::For => "notation_date",
				],
				crate::locale::get_simple(&model.locale, "day"),
			],
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
			button![
				attrs![
					At::Type => "button",
					At::Id => "next",
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
			button![
				attrs![
					At::Type => "button",
					At::Id => "set_today",
				],
				ev(Ev::Click, |_| crate::message::Message::SetDateToday),
				crate::locale::get_simple(&model.locale, "today"),
			],
		],
		article![
			notation_subjects,
			p![
				C!["call_to_action"],
				input![
					attrs![
						At::Type => "submit",
						At::Value => crate::locale::get_simple(&model.locale, "save"),
					],
					C!["primary", "tw-col-span-6"],
					ev(Ev::Click, |_| crate::message::Message::SaveRate),
				],
				input![
					attrs![
						At::Type => "reset",
						At::Value => crate::locale::get_simple(&model.locale, "reset"),
					],
					C!["tw-col-span-6"],
					ev(Ev::Click, |_| crate::message::Message::ResetSubjects),
				],
			],
		],
	];
}
