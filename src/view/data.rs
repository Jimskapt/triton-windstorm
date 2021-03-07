use seed::{prelude::*, *};

#[derive(serde::Serialize)]
struct Export {
	generator_name: String,
	generator_version: String,
	subjects: Vec<crate::model::Subject>,
	records: Vec<crate::model::Rate>,
}

pub fn view(model: &crate::model::Model) -> Node<crate::messages::Message> {
	let records = model.records.values().cloned().collect();

	let current_data = Export {
		generator_name: String::from(env!("CARGO_PKG_NAME")),
		generator_version: String::from(env!("CARGO_PKG_VERSION")),
		subjects: model.subjects.values().cloned().collect(),
		records,
	};

	return div![
		h2!(crate::locale::get_simple(&model.locale, "your-data")),
		/*
		article![
			a![
				attrs![
					At::Type => "button",
					At::Href => "#/your-data/export"
				],
				crate::locale::get_simple(&model.locale, "export"),
			],
		],
		*/
		h3![match model.panel {
			crate::model::AppPanel::ExportData =>
				crate::locale::get_simple(&model.locale, "export-your-data"),
			_ => String::from("ERROR"),
		}],
		article![
			p![label![
				input![
					attrs![
						At::Type => "checkbox",
						At::Checked => model.pretty_export.as_at_value(),
					],
					ev(Ev::Change, |event| {
						let target = seed::prelude::wasm_bindgen::JsCast::dyn_into::<
							seed::prelude::web_sys::HtmlInputElement,
						>(event.target().unwrap())
						.unwrap();

						crate::messages::Message::Data(crate::messages::data::Message::SetPretty(
							target.checked(),
						))
					}),
				],
				format!(" {}", crate::locale::get_simple(&model.locale, "readable")),
			],],
			textarea![
				attrs![
					At::Rows => 15,
				],
				style![
					St::LineBreak => "anywhere",
				],
				C!["tw-w-full"],
				match model.pretty_export {
					true => serde_json::to_string_pretty(&current_data),
					false => serde_json::to_string(&current_data),
				}
				.unwrap(),
			]
		],
	];
}
