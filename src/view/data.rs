use seed::{prelude::*, *};

#[derive(serde::Serialize)]
struct Export {
	generator_name: String,
	generator_version: String,
	subjects: Vec<crate::model::Subject>,
	saves: Vec<crate::model::Rate>,
}

pub fn view(model: &crate::model::Model) -> Node<crate::message::Message> {
	let saves = model.saves.values().cloned().collect();

	let current_data = Export {
		generator_name: String::from(env!("CARGO_PKG_NAME")),
		generator_version: String::from(env!("CARGO_PKG_VERSION")),
		subjects: model.pending_rate.subjects.clone(),
		saves,
	};

	return div![
		h2!(crate::locale::get_simple(&model.locale, "your-data")),
		/*
		article![
			button![
				attrs![
					At::Type => "button",
				],
				crate::locale::get_simple(&model.locale, "export"),
				ev(Ev::Click, |_| crate::message::Message::GoToPanel {
					panel: crate::model::AppPanel::ExportData
				}),
			],
		],
		*/
		h3![match model.panel {
			crate::model::AppPanel::ExportData =>
				crate::locale::get_simple(&model.locale, "export-your-data"),
			_ => String::from("ERROR"),
		}],
		article![textarea![
			attrs![
				At::Value => serde_json::to_string_pretty(&current_data).unwrap(),
				At::Rows => 15,
			],
			C!["tw-w-full"],
		]],
	];
}
