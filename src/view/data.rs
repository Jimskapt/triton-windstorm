use seed::{prelude::*, *};

pub fn view(model: &crate::model::Model) -> Node<crate::messages::Message> {
	let records = model.records.values().cloned().collect();

	let current_data = crate::model::Export {
		generator_name: String::from(env!("CARGO_PKG_NAME")),
		generator_version: String::from(env!("CARGO_PKG_VERSION")),
		subjects: Some(model.subjects.values().cloned().collect()),
		records: Some(records),
	};

	return div![
		h2!(crate::locale::get_simple(&model.locale, "your-data")),
		article![nav![
			a![
				attrs![
					At::Type => "button",
					At::Href => "#/your-data/export"
				],
				crate::locale::get_simple(&model.locale, "export"),
			],
			a![
				attrs![
					At::Type => "button",
					At::Href => "#/your-data/import"
				],
				crate::locale::get_simple(&model.locale, "import"),
			],
		],],
		h3![match model.panel {
			crate::model::AppPanel::ExportData =>
				crate::locale::get_simple(&model.locale, "export-your-data"),
			crate::model::AppPanel::ImportData =>
				crate::locale::get_simple(&model.locale, "import-your-data"),
			_ => String::from("ERROR"),
		}],
		article![match model.panel {
			crate::model::AppPanel::ExportData => div![
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

							crate::messages::Message::Data(
								crate::messages::data::Message::SetPretty(target.checked()),
							)
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
					C!["tw-w-full", "tw-text-xs"],
					match model.pretty_export {
						true => serde_json::to_string_pretty(&current_data),
						false => serde_json::to_string(&current_data),
					}
					.unwrap(),
				],
			],
			crate::model::AppPanel::ImportData => div![
				textarea![
					attrs![
						At::Rows => 15,
					],
					style![
						St::LineBreak => "anywhere",
					],
					C!["tw-w-full", "tw-text-xs"],
					input_ev(Ev::Input, |value| crate::messages::Message::Data(
						crate::messages::data::Message::SetPendingImport(value)
					)),
				],
				p![
					C!["call_to_action"],
					button![
						C!["tw-col-span-6"],
						format!("{} ...", crate::locale::get_simple(&model.locale, "add-to")),
						ev(Ev::Click, |_| crate::messages::Message::Data(
							crate::messages::data::Message::DoImport(false)
						)),
					],
					button![
						C!["tw-col-span-6"],
						format!(
							"{} ...",
							crate::locale::get_simple(&model.locale, "replace-all")
						),
						ev(Ev::Click, |_| crate::messages::Message::Data(
							crate::messages::data::Message::DoImport(true)
						)),
					],
				],
				p![label![
					C!["tw-text-center", "tw-block", "tw-mt-1"],
					format!(
						"... {}",
						crate::locale::get_simple(&model.locale, "existing-data")
					),
				],],
			],
			_ => p!["ERROR"],
		},],
	];
}
