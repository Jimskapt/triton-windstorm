use seed::{prelude::*, *};

pub fn view(model: &crate::model::Model) -> Node<crate::messages::Message> {
	return div![
		h2![crate::locale::get_simple(&model.locale, "about")],
		article![
			style![
				St::TextAlign => "center",
			],
			h3!["Triton Windstorm"],
			p![&format!(
				"{} : V{}",
				crate::locale::get_simple(&model.locale, "app-version"),
				env!("CARGO_PKG_VERSION"),
			),],
		],
		article![
			style![
				St::TextAlign => "center",
			],
			p![
				"Made with \u{2764} by ",
				a![
					attrs![
						At::Href => "https://jimskapt.com/"
					],
					"Thomas RAMIREZ",
				],
				" in France \u{1f1eb}\u{1f1f7}",
			],
		],
		article![
			style![
				St::TextAlign => "center",
			],
			p![
				crate::locale::get_simple(&model.locale, "open-source-code-on"),
				" ",
				a![
					attrs![
						At::Href => "https://github.com/Jimskapt/triton-windstorm"
					],
					"GitHub",
				],
			],
		],
	];
}
