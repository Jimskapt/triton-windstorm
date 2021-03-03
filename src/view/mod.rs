use seed::{prelude::*, *};

mod index;
mod settings;

pub fn view(model: &crate::model::Model) -> Node<crate::message::Message> {
	return div![
		IF![
			model.dark_theme => C![
				"tw-dark",
			]
		],
		IF![
			model.show_unallowed_save => p![
				C!["message", "warning"],
				span![crate::locale::get_simple(&model.locale, "unallowed-save")],
				raw!(" "),
				button![
					C!["primary"],
					crate::locale::get_simple(&model.locale, "allow"),
					ev(Ev::Click, |_| crate::message::Message::AllowStorage),
				],
				raw!(" "),
				button![
					crate::locale::get_simple(&model.locale, "dismiss"),
					ev(Ev::Click, |_| crate::message::Message::DismissStorageWarning),
				],
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
					ev(Ev::Click, |_| crate::message::Message::GoToPanel {
						panel: crate::model::AppPanel::Index
					}),
				],
				raw!(" "),
				a![
					attrs![
						At::Href => "#/settings",
					],
					crate::locale::get_simple(&model.locale, "settings"),
					ev(Ev::Click, |_| crate::message::Message::GoToPanel {
						panel: crate::model::AppPanel::Settings
					}),
				],
			],
		],
		main![match model.panel {
			crate::model::AppPanel::Index => index::view(model),
			crate::model::AppPanel::Settings => settings::view(model),
		},],
	];
}
