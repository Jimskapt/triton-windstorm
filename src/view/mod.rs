use seed::{prelude::*, *};

mod about;
mod data;
mod graphs;
mod index;
mod settings;

pub fn view(model: &crate::model::Model) -> Node<crate::messages::Message> {
	return div![
		IF![
			model.dark_theme => C![
				"tw-dark",
			]
		],
		header![nav![
			a![
				attrs![
					At::Href => "#/index",
				],
				format!(
					"\u{1f3e0} {}",
					crate::locale::get_simple(&model.locale, "index")
				),
			],
			a![
				attrs![
					At::Href => "#/settings",
				],
				format!(
					"\u{2699}\u{fe0f} {}",
					crate::locale::get_simple(&model.locale, "settings")
				),
			],
			a![
				attrs![
					At::Href => "#/your-data/export",
				],
				format!(
					"\u{1f4be} {}",
					crate::locale::get_simple(&model.locale, "your-data")
				),
			],
			a![
				attrs![
					At::Href => "#/graphs",
				],
				format!(
					"\u{1f4c8} {}",
					crate::locale::get_simple(&model.locale, "graphics")
				),
			],
			a![
				attrs![
					At::Href => "#/about",
				],
				format!(
					"\u{2139}\u{fe0f} {}",
					crate::locale::get_simple(&model.locale, "about")
				),
			],
		],],
		main![
			IF![
				model.show_unallowed_save => article![
					C!["message", "warning"],
					span![crate::locale::get_simple(&model.locale, "unallowed-save")],
					p![
						C!["call_to_action"],
						button![
							C!["primary", "tw-col-span-6"],
							crate::locale::get_simple(&model.locale, "allow"),
							ev(Ev::Click, |_| crate::messages::Message::AllowStorage),
						],
						button![
							C!["tw-col-span-6"],
							crate::locale::get_simple(&model.locale, "dismiss"),
							ev(Ev::Click, |_| crate::messages::Message::DismissStorageWarning),
						],
					],
				]
			],
			match model.panel {
				crate::model::AppPanel::Index => index::view(model),
				crate::model::AppPanel::Settings => settings::view(model),
				crate::model::AppPanel::ExportData | crate::model::AppPanel::ImportData =>
					data::view(model),
				crate::model::AppPanel::Graphics => graphs::view(model),
				crate::model::AppPanel::About => about::view(model),
			},
		],
	];
}
