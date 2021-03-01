const DEFAULT_LOCALE: &str = "en-US";

const AVAILABLE_LOCALES: &[(&str, &str)] = &[
	("fr-FR", include_str!("fr-FR.flu")),
	("en-US", include_str!("en-US.flu")),
];

pub fn get_bundle(selected_locale: &str) -> fluent::FluentBundle<fluent::FluentResource> {
	let default = (selected_locale, "");
	let (key, value) = match AVAILABLE_LOCALES
		.iter()
		.find(|(locale, _)| locale == &selected_locale)
	{
		Some(res) => res,
		None => {
			seed::error!(
				"locale `{}` was not found, fallback to `{}`",
				selected_locale,
				DEFAULT_LOCALE
			);

			AVAILABLE_LOCALES
				.iter()
				.find(|(locale, _)| locale == &DEFAULT_LOCALE)
				.unwrap_or(&default)
		}
	};

	let selected_fluent = fluent::FluentResource::try_new(String::from(*value)).unwrap();

	let default_fluent = fluent::FluentResource::try_new(String::from(
		AVAILABLE_LOCALES
			.iter()
			.find(|(locale, _)| locale == &DEFAULT_LOCALE)
			.unwrap_or(&default)
			.1,
	))
	.unwrap();

	let mut fluent_bundle = fluent::FluentBundle::new(vec![
		key.parse::<unic_langid::LanguageIdentifier>().unwrap(),
		DEFAULT_LOCALE
			.parse::<unic_langid::LanguageIdentifier>()
			.unwrap(),
	]);
	fluent_bundle.add_resource(selected_fluent).unwrap();

	// it's important to add default AFTER selected to not override existing keys in bundle :
	fluent_bundle.add_resource(default_fluent).ok();

	return fluent_bundle;
}

pub fn get_simple(bundle: &fluent::FluentBundle<fluent::FluentResource>, key: &str) -> String {
	return match bundle.get_message(key) {
		Some(val) => bundle
			.format_pattern(val.value().unwrap(), None, &mut vec![])
			.into_owned(),
		None => {
			let val = format!("{{locale:{}}}", key);
			seed::error![format!(
				"locale : can not find key `{}` in locales, fallback to value `{}`",
				key, val
			)];
			val
		}
	};
}
