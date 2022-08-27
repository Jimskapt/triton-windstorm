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

	pub database_account_string: String,
	pub database_remote: Option<pontus_onyx::client::ClientRemote>,
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
	#[serde(skip)]
	pub source: SubjectSource,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SubjectSource {
	Generated,
	User,
}
impl Default for SubjectSource {
	fn default() -> Self {
		Self::User
	}
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

// TODO : warn user when leaving document where there is unsaved data (no localStorage, no remoteStorage)

pub fn init(
	url: seed::Url,
	orders: &mut impl seed::prelude::Orders<crate::messages::Message>,
) -> Model {
	orders.subscribe(crate::messages::Message::UrlChanged);
	orders.send_msg(crate::messages::Message::UrlChanged(
		seed::prelude::subs::UrlChanged(url.clone()),
	));

	let window = seed::prelude::web_sys::window().unwrap();

	let storage = window.local_storage().unwrap().unwrap();

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

	if allowed_save {
		if let Ok(length) = storage.length() {
			let mut migrated_subjects: std::collections::HashMap<String, crate::model::Subject> =
				std::collections::HashMap::new();

			for i in 0..length {
				let key = storage.key(i).unwrap().unwrap();

				if let Some(temp_next) =
					key.strip_prefix(&format!("{}subject_", crate::storage::STORAGE_PREFIX))
				{
					let (id, field) = match temp_next.strip_suffix("_name") {
						Some(id) => (Some(id), Some("name")),
						None => match temp_next.strip_suffix("_max") {
							Some(id) => (Some(id), Some("max")),
							None => match temp_next.strip_suffix("_steps") {
								Some(id) => (Some(id), Some("steps")),
								None => (None, None),
							},
						},
					};

					if let Some(id) = id {
						let mut subject = match migrated_subjects.get_mut(id) {
							Some(subject) => subject,
							None => {
								migrated_subjects.insert(
									String::from(id),
									Subject {
										id: String::from(id),
										name: String::from("mood"),
										value: None,
										max: 5.0,
										observations: None,
										steps: 1.0,
										source: SubjectSource::User,
									},
								);

								migrated_subjects.get_mut(id).unwrap()
							}
						};

						let value = storage.get(&key).unwrap().unwrap();

						if let Some(field) = field {
							if field == "name" {
								(*subject).name = value;
							} else if field == "max" {
								match value.parse::<f64>() {
									Ok(value) => {
										(*subject).max = value;
									}
									Err(err) => {
										seed::error!(err);
									}
								}
							} else if field == "steps" {
								match value.parse::<f64>() {
									Ok(value) => {
										(*subject).steps = value;
									}
									Err(err) => {
										seed::error!(err);
									}
								}
							}
						}
					}
				}
			}

			for (id, subject) in migrated_subjects {
				if !subject.name.is_empty() {
					if storage
						.set_item(
							&format!("{}subject_{id}", crate::storage::STORAGE_PREFIX),
							&serde_json::to_string(&subject).unwrap(),
						)
						.is_ok()
					{
						seed::log!(&format!("subject {id} has been migrated"));

						storage
							.remove_item(&format!(
								"{}subject_{id}_name",
								crate::storage::STORAGE_PREFIX
							))
							.ok();
						storage
							.remove_item(&format!(
								"{}subject_{id}_max",
								crate::storage::STORAGE_PREFIX
							))
							.ok();
						storage
							.remove_item(&format!(
								"{}subject_{id}_steps",
								crate::storage::STORAGE_PREFIX
							))
							.ok();
					}
				}
			}
		}
	}

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
				let id = temp_next;
				match serde_json::from_str(&value) {
					Ok(value) => {
						subjects.insert(String::from(id), value);
					}
					Err(err) => {
						seed::error!(err);
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
				source: SubjectSource::Generated,
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

	let database_account_string = if let Some(path) = url.hash_path().get(0) {
		if path.starts_with("db_register") {
			let url: Result<seed::browser::url::Url, String> = std::str::FromStr::from_str(path);

			let db_account = if let Ok(url) = url {
				String::from(
					match url
						.search()
						.get("db_account")
						.and_then(|values| values.get(0))
					{
						Some(db_account) => db_account,
						None => "",
					},
				)
			} else {
				String::new()
			};

			if !db_account.is_empty() {
				orders.after_next_render(|_| {
					crate::messages::Message::Database(
						crate::messages::database::Message::BuildRemote(Some(Box::new(
							crate::messages::Message::Database(
								crate::messages::database::Message::SyncDatabase,
							),
						))),
					)
				});
			}

			db_account
		} else {
			String::new()
		}
	} else {
		String::new()
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
		database_account_string,
		database_remote: None,
	};
}
