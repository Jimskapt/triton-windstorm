pub struct Model {
	pub locale: fluent::FluentBundle<fluent::FluentResource>,
	pub allowed_save: bool,
	pub show_unallowed_save: bool,
	pub pending_rate: Rate,
	pub panel: AppPanel,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Rate {
	pub date: chrono::naive::NaiveDate,
	pub subjects: Vec<Subject>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Subject {
	pub id: String,
	pub name: String,
	pub value: Option<f64>,
	pub max: f64,
	pub observations: Option<String>,
}

pub enum AppPanel {
	Index,
	Settings,
}

pub fn init(
	_url: seed::Url,
	orders: &mut impl seed::prelude::Orders<crate::message::Message>,
) -> Model {
	let storage = seed::prelude::web_sys::window()
		.unwrap()
		.local_storage()
		.unwrap()
		.unwrap();

	let locale = crate::locale::get_bundle(&match storage
		.get_item(&format!("{}locale", crate::storage::STORAGE_PREFIX))
		.unwrap_or(None)
	{
		Some(locale) => locale,
		None => String::from(
			match seed::prelude::web_sys::window()
				.unwrap()
				.navigator()
				.language()
			{
				Some(lang) if lang == "fr" => "fr-FR",
				Some(lang) if lang == "fr-FR" => "fr-FR",
				_ => "en-US",
			},
		),
	});

	let allowed_save = crate::storage::get_allowed(&storage) == "true";
	let show_unallowed_save = crate::storage::get_allowed(&storage) != "true";

	let today = chrono::offset::Local::today();
	let pending_rate = Rate {
		date: chrono::NaiveDate::from_ymd(
			chrono::Datelike::year(&today),
			chrono::Datelike::month(&today),
			chrono::Datelike::day(&today),
		),
		subjects: vec![],
	};

	orders.after_next_render(|_| crate::message::Message::SetDateToday);

	return Model {
		locale,
		allowed_save,
		show_unallowed_save,
		pending_rate,
		panel: AppPanel::Index,
	};
}
