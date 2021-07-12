use seed::{prelude::*, *};

use std::convert::TryFrom;

pub fn view(model: &crate::model::Model) -> Node<crate::messages::Message> {
	let notation_subjects = model.pending_rate.subjects.iter().map(|subject| {
		let id_value = subject.id.clone();
		let id_observation = subject.id.clone();

		if subject.name.trim() != "" {
			let mut leading_zeroes: Option<usize> = None;
			let mut decimals: Option<usize> = None;
			for i in 0..i32::MAX {
				if leading_zeroes.is_none() && (subject.max as f32) / f32::powi(10.0, i) < 10.0 {
					if let Ok(i) = usize::try_from(i) {
						leading_zeroes = Some(i + 1);
					}
				}

				let temp = f32::abs(subject.steps as f32 - subject.steps.floor() as f32)
					* f32::powi(10.0, i);
				if decimals.is_none() && f32::abs(temp - temp.floor()) == 0.0 {
					if let Ok(i) = usize::try_from(i) {
						decimals = Some(i);
					}
				}

				if leading_zeroes.is_some() && decimals.is_some() {
					break;
				}
			}

			return article![
				C!["subject"],
				div![
					C!["rate"],
					label![
						attrs![
							At::For => format!("rate-{}", subject.id),
						],
						&subject.name,
					],
					raw![&match subject.value {
						Some(value) => format!(
							"{0:01$.2$} ",
							value,
							leading_zeroes.unwrap_or(0) + 1 + decimals.unwrap_or(0),
							decimals.unwrap_or(0)
						),
						None => format!(
							"{:-<1$} ",
							"",
							leading_zeroes.unwrap_or(0) + 1 + decimals.unwrap_or(0)
						),
					}],
					input![
						attrs![
							At::Type => "range",
							At::Min => 0,
							At::Step => subject.steps,
							At::Max => subject.max,
							At::Value => subject.value.unwrap_or(subject.max),
							At::Id => format!("rate-{}", subject.id),
						],
						input_ev(Ev::Input, move |value| {
							crate::messages::Message::Index(
								crate::messages::index::Message::SetSubjectValue {
									id: id_value,
									value: Some(value),
								},
							)
						}),
					],
					raw![&format!(
						" {0:01$.2$}",
						subject.max,
						leading_zeroes.unwrap_or(0) + 1 + decimals.unwrap_or(0),
						decimals.unwrap_or(0)
					)],
				],
				div![
					C!["observation"],
					textarea![
						attrs![
							At::Type => "text",
							At::Placeholder => crate::locale::get_simple(&model.locale, "observation"),
							At::Rows => 1,
							At::Value => subject.observations.clone().unwrap_or_default(),
						],
						input_ev(Ev::Input, move |value| {
							crate::messages::Message::Index(
								crate::messages::index::Message::SetSubjectObservation {
									id: id_observation,
									observation: Some(value),
								},
							)
						}),
					],
				],
			];
		}

		return seed::div![];
	});

	let date_for_previous = model.pending_rate.date;
	let date_for_next = model.pending_rate.date;

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
					crate::messages::Message::Index(crate::messages::index::Message::SetRateDay {
						day: format!(
							"{}",
							(date_for_previous - chrono::Duration::days(1)).format("%Y-%m-%d")
						),
					})
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

					crate::messages::Message::Index(crate::messages::index::Message::SetRateDay {
						day: target.value(),
					})
				}),
			],
			button![
				attrs![
					At::Type => "button",
					At::Id => "next",
				],
				">",
				ev(Ev::Click, move |_| {
					crate::messages::Message::Index(crate::messages::index::Message::SetRateDay {
						day: format!(
							"{}",
							(date_for_next + chrono::Duration::days(1)).format("%Y-%m-%d")
						),
					})
				}),
			],
			button![
				attrs![
					At::Type => "button",
					At::Id => "set_today",
				],
				ev(Ev::Click, |_| crate::messages::Message::Index(
					crate::messages::index::Message::SetDateToday
				)),
				crate::locale::get_simple(&model.locale, "today"),
			],
		],
		notation_subjects,
		article![p![
			C!["call_to_action"],
			input![
				attrs![
					At::Type => "submit",
					At::Value => format!("\u{1f4be} {}", crate::locale::get_simple(&model.locale, "save")),
				],
				C!["primary", "tw-col-span-6"],
				ev(Ev::Click, |_| crate::messages::Message::Index(
					crate::messages::index::Message::SaveRate
				)),
			],
			input![
				attrs![
					At::Type => "reset",
					At::Value => format!("\u{1f519} {}", crate::locale::get_simple(&model.locale, "reset")),
				],
				C!["tw-col-span-6"],
				ev(Ev::Click, |_| crate::messages::Message::Index(
					crate::messages::index::Message::ResetSubjects
				)),
			],
		],],
	];
}
