pub struct Model {
	pub locale: fluent::FluentBundle<fluent::FluentResource>,
	pub dark_theme: bool,

	pub allowed_save: bool,
	pub show_unallowed_save: bool,

	pub panel: AppPanel,

	pub subjects: std::collections::BTreeMap<String, Subject>,
	pub pending_rate: Rate,
	pub records: std::collections::BTreeMap<String, Rate>,

	pub pretty_export: bool,

	pub pending_import: String,

	pub do_render_graphics: bool,
	pub graphs_canvas: seed::prelude::ElRef<seed::prelude::web_sys::HtmlCanvasElement>,
	pub historical_subjects: std::collections::BTreeMap<String, HistoricalSubject>,
	pub graph_start: Option<chrono::naive::NaiveDate>,
	pub graph_end: Option<chrono::naive::NaiveDate>,
	pub show_points: bool,
	pub show_grid: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
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
	#[serde(default)]
	pub steps: f64,
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct HistoricalSubject {
	pub checked: bool,
	pub color: String,
	pub find_ids: Vec<String>,
	pub min: Option<f64>,
	pub max: Option<f64>,
	pub average: Option<f64>,
	pub deviation: Option<f64>,
	pub average_error: Option<f64>,
}

impl Eq for HistoricalSubject {}
impl Ord for HistoricalSubject {
	fn cmp(&self, _: &Self) -> std::cmp::Ordering {
		std::cmp::Ordering::Greater
	}
}

pub enum AppPanel {
	Index,
	Settings,
	ExportData,
	ImportData,
	Graphics,
	About,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Export {
	pub generator_name: String,
	pub generator_version: String,
	pub subjects: Option<Vec<crate::model::Subject>>,
	pub records: Option<Vec<crate::model::Rate>>,
}

pub fn init(
	url: seed::Url,
	orders: &mut impl seed::prelude::Orders<crate::messages::Message>,
) -> Model {
	orders.subscribe(crate::messages::Message::UrlChanged);
	orders.send_msg(crate::messages::Message::UrlChanged(
		seed::prelude::subs::UrlChanged(url),
	));

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

	let dark = match storage.get_item(&format!("{}dark_theme", crate::storage::STORAGE_PREFIX)) {
		Ok(Some(value)) => value == "true",
		_ => {
			match web_sys::window()
				.unwrap()
				.match_media("(prefers-color-scheme: dark)")
			{
				Ok(Some(res)) => res.matches(),
				_ => false,
			}
		}
	};
	orders.after_next_render(move |_| {
		crate::messages::Message::Settings(crate::messages::settings::Message::SetDarkTheme {
			value: dark,
		})
	});

	let today = chrono::offset::Local::today();
	let pending_rate = Rate {
		date: chrono::NaiveDate::from_ymd(
			chrono::Datelike::year(&today),
			chrono::Datelike::month(&today),
			chrono::Datelike::day(&today),
		),
		subjects: vec![],
	};

	orders.after_next_render(|_| {
		crate::messages::Message::Index(crate::messages::index::Message::SetDateToday)
	});

	let mut records = std::collections::BTreeMap::new();
	let mut subjects: std::collections::BTreeMap<String, Subject> =
		std::collections::BTreeMap::new();
	if let Ok(length) = storage.length() {
		for i in 0..length {
			let key = storage.key(i).unwrap().unwrap();
			let value = storage.get(&key).unwrap().unwrap();

			if let Some(temp_next) =
				str::strip_prefix(&key, &format!("{}subject_", crate::storage::STORAGE_PREFIX))
			{
				if let Some(id) = temp_next.strip_suffix("_name") {
					let id = String::from(id);
					match subjects.get_mut(&id) {
						Some(subject) => {
							(*subject).name = value.clone();
						}
						None => {
							subjects.insert(
								id.clone(),
								crate::model::Subject {
									id,
									name: value.clone(),
									max: 5.0,
									value: None,
									observations: None,
									steps: 1.0,
								},
							);
						}
					}
				}

				if let Some(id) = temp_next.strip_suffix("_steps") {
					let id = String::from(id);
					match subjects.get_mut(&id) {
						Some(subject) => {
							(*subject).steps = value.clone().parse().unwrap_or(0.1);
						}
						None => {
							subjects.insert(
								id.clone(),
								crate::model::Subject {
									id,
									name: String::new(),
									max: 5.0,
									value: None,
									observations: None,
									steps: value.clone().parse().unwrap_or(0.1),
								},
							);
						}
					}
				}

				if let Some(id) = temp_next.strip_suffix("_max") {
					match subjects.get_mut(id) {
						Some(subject) => {
							subject.max = value.parse().unwrap_or(5.0);
						}
						None => {
							let id = String::from(id);
							subjects.insert(
								id.clone(),
								crate::model::Subject {
									id,
									name: String::new(),
									max: value.parse().unwrap_or(5.0),
									value: None,
									observations: None,
									steps: 1.0,
								},
							);
						}
					}
				}
			} else if let Some(temp_next) =
				str::strip_prefix(&key, &format!("{}record_", crate::storage::STORAGE_PREFIX))
			{
				records.insert(
					String::from(temp_next),
					serde_json::from_str(&value).unwrap(),
				);
			}
		}
	}
	if subjects.is_empty() {
		let id = format!("{}", uuid::Uuid::new_v4());
		subjects.insert(
			id.clone(),
			crate::model::Subject {
				id,
				name: String::from("mood"),
				max: 5.0,
				value: None,
				observations: None,
				steps: 1.0,
			},
		);
	}

	let mut graph_start = None;
	if let Some(first) = records.keys().into_iter().next() {
		if let Ok(first) = chrono::NaiveDate::parse_from_str(first, "%Y-%m-%d") {
			if let Some(last) = records.keys().into_iter().last() {
				if let Ok(last) = chrono::NaiveDate::parse_from_str(last, "%Y-%m-%d") {
					let limit = chrono::Duration::days(8);
					if last - first > limit {
						graph_start = Some(last - limit);
					}
				}
			}
		}
	};

	return Model {
		locale,
		allowed_save,
		show_unallowed_save,
		pending_rate,
		subjects,
		panel: AppPanel::Index,
		dark_theme: false,
		records,
		pretty_export: true,
		pending_import: String::new(),
		do_render_graphics: false,
		graphs_canvas: seed::prelude::ElRef::default(),
		historical_subjects: std::collections::BTreeMap::new(),
		graph_start,
		graph_end: None,
		show_points: true,
		show_grid: true,
	};
}
