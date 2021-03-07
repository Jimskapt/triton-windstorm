use seed::{prelude::*, *};

mod data;
mod index;
mod settings;

pub fn view(model: &crate::model::Model) -> Node<crate::messages::Message> {
	return div![
		IF![
			model.dark_theme => C![
				"tw-dark",
			]
		],
		header![
			h1!["Triton Windstorm"],
			nav![
				a![
					attrs![
						At::Href => "#/index",
					],
					crate::locale::get_simple(&model.locale, "index"),
				],
				a![
					attrs![
						At::Href => "#/settings",
					],
					crate::locale::get_simple(&model.locale, "settings"),
				],
				a![
					attrs![
						At::Href => "#/your-data/export",
					],
					crate::locale::get_simple(&model.locale, "your-data"),
				],
			],
		],
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
				crate::model::AppPanel::ExportData => data::view(model),
			},
		],
	];
}
