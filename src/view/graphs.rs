use seed::{prelude::*, *};

pub fn view(model: &crate::model::Model) -> Node<crate::messages::Message> {
	let subjects_list: Vec<Node<crate::messages::Message>> = model
		.historical_subjects
		.iter()
		.map(|(name, subject)| {
			let name_for_event = name.clone();
			label![
				C!["tw-col-span-12", "sm:tw-col-span-6", "md:tw-col-span-4",],
				input![
					attrs! {
						At::Type => "checkbox",
						At::Name => format!("subject-{}", name),
						At::Checked => subject.checked.as_at_value(),
					},
					ev(Ev::Change, |event| {
						let target = wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlInputElement>(
							event.target().unwrap(),
						)
						.unwrap();

						crate::messages::Message::Graphs(
							crate::messages::graphs::Message::SetSubjectValue {
								name: name_for_event,
								value: target.checked(),
							},
						)
					}),
				],
				format!(" {}", name),
			]
		})
		.collect();

	return div![
		h2!(crate::locale::get_simple(&model.locale, "graphics")),
		article![canvas![
			el_ref(&model.graphs_canvas),
			attrs! {
				At::Width => px(400),
				At::Height => px(400),
			},
			style![
				St::Width => "100%",
				St::Height => "400px",
			],
		],],
		h3!(crate::locale::get_simple(&model.locale, "settings")),
		article![
			C!["tw-grid", "tw-grid-cols-12",],
			label![
				C!["tw-col-span-12", "sm:tw-col-span-6",],
				attrs![
					At::For => "start_date",
				],
				crate::locale::get_simple(&model.locale, "start"),
			],
			input![
				C!["tw-col-span-12", "sm:tw-col-span-6",],
				attrs![
					At::Type => "date",
					At::Id => "start_date",
					At::Value => match model.graph_start {
						Some(value) => format!("{}", value.format("%Y-%m-%d")),
						None => String::new(),
					},
				],
				ev(Ev::Change, |event| {
					let target = wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlInputElement>(
						event.target().unwrap(),
					)
					.unwrap();

					let value = target.value();
					if value == "" {
						crate::messages::Message::Graphs(
							crate::messages::graphs::Message::SetStart(None),
						)
					} else {
						crate::messages::Message::Graphs(
							crate::messages::graphs::Message::SetStart(Some(value)),
						)
					}
				}),
			],
			label![
				C!["tw-col-span-12", "sm:tw-col-span-6",],
				attrs![
					At::For => "end_date",
				],
				crate::locale::get_simple(&model.locale, "end"),
			],
			input![
				C!["tw-col-span-12", "sm:tw-col-span-6",],
				attrs![
					At::Type => "date",
					At::Id => "end_date",
					At::Value => match model.graph_end {
						Some(value) => format!("{}", value.format("%Y-%m-%d")),
						None => String::new(),
					},
				],
				ev(Ev::Change, |event| {
					let target = wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlInputElement>(
						event.target().unwrap(),
					)
					.unwrap();

					let value = target.value();
					if value == "" {
						crate::messages::Message::Graphs(crate::messages::graphs::Message::SetEnd(
							None,
						))
					} else {
						crate::messages::Message::Graphs(crate::messages::graphs::Message::SetEnd(
							Some(value),
						))
					}
				}),
			],
			label![
				C!["tw-col-span-12",],
				crate::locale::get_simple(&model.locale, "subjects"),
			],
			div![
				C!["tw-col-span-12", "tw-grid", "tw-grid-cols-12",],
				button![
					C!["tw-col-span-6",],
					attrs![
						At::Type => "button",
					],
					crate::locale::get_simple(&model.locale, "all"),
					ev(Ev::Click, |_| {
						crate::messages::Message::Graphs(
							crate::messages::graphs::Message::SetAllSubjects(true),
						)
					}),
				],
				button![
					C!["tw-col-span-6",],
					attrs![
						At::Type => "button",
					],
					crate::locale::get_simple(&model.locale, "none"),
					ev(Ev::Click, |_| {
						crate::messages::Message::Graphs(
							crate::messages::graphs::Message::SetAllSubjects(false),
						)
					}),
				],
			],
			subjects_list,
		]
	];
}
