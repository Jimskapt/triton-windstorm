const ALPHABET: &[&str] = &[
	"0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
];

pub enum Message {
	ComputeHistoricalSubjects,
	UpdateGraph,
	SetSubjectValue { name: String, value: bool },
	SetAllSubjects(bool),
	SetStart(Option<String>),
	SetEnd(Option<String>),
	SetShowPoints(bool),
	SetShowGrid(bool),
}

pub fn update(
	message: Message,
	model: &mut crate::model::Model,
	orders: &mut impl seed::prelude::Orders<crate::messages::Message>,
) {
	match message {
		Message::ComputeHistoricalSubjects => {
			let mut temp_subjects = model.subjects.clone();
			for record in model.records.values() {
				for subject in &record.subjects {
					if temp_subjects.get(&subject.id).is_none()
						&& temp_subjects
							.iter()
							.find(|(_, s)| s.name == subject.name)
							.is_none()
					{
						temp_subjects.insert(subject.id.clone(), subject.clone());
					}
				}
			}
			// TODO : cleanup of empty names ?

			model.historical_subjects = std::collections::BTreeMap::new();

			temp_subjects.values().for_each(|subject| {
				let mut color = String::from("#");
				for _ in 0..6 {
					color +=
						rand::seq::IteratorRandom::choose(ALPHABET.iter(), &mut rand::thread_rng())
							.unwrap();
				}

				match model.historical_subjects.get_mut(&subject.name) {
					Some(find) => {
						find.find_ids.push(subject.id.clone());
					}
					None => {
						model.historical_subjects.insert(
							subject.name.clone(),
							crate::model::HistoricalSubject {
								checked: true,
								color,
								find_ids: vec![subject.id.clone()],
							},
						);
					}
				}
			});
		}
		Message::UpdateGraph => {
			if let Some(canvas) = model.graphs_canvas.get() {
				canvas.set_width(canvas.client_width() as u32);
				canvas.set_height(canvas.client_height() as u32);

				let margin = 30.0;
				let available_x = f64::from(canvas.client_width()) - 2.0 * margin;
				let available_y = f64::from(canvas.client_height()) - 2.0 * margin;
				let yesterday_str = format!(
					"{}",
					(chrono::offset::Local::today() - chrono::Duration::days(1)).format("%Y-%m-%d")
				);
				let today_str = format!("{}", chrono::offset::Local::today().format("%Y-%m-%d"));
				let start_date = model.graph_start.unwrap_or_else(|| {
					chrono::NaiveDate::parse_from_str(
						model.records.keys().nth(0).unwrap_or(&yesterday_str),
						"%Y-%m-%d",
					)
					.unwrap()
				});
				let end_date = model.graph_end.unwrap_or_else(|| {
					chrono::NaiveDate::parse_from_str(
						model.records.keys().last().unwrap_or(&today_str),
						"%Y-%m-%d",
					)
					.unwrap()
				});
				let date_range = end_date - start_date;
				let x_spacing = f64::abs(available_x / date_range.num_days() as f64);
				let mut points = std::collections::BTreeMap::new();
				for (subject_name, subject) in &model.historical_subjects {
					let subject_ids = subject.find_ids.clone();
					if subject.checked {
						let mut temp = std::collections::BTreeMap::new();
						for (day, rate) in &model.records {
							let mut process_rate = |rate: &crate::model::Subject| {
								let max = rate.max;
								if let Some(rate) = rate.value {
									temp.insert(
										((chrono::NaiveDate::parse_from_str(day, "%Y-%m-%d")
											.unwrap() - start_date)
											.num_days() as f64 * x_spacing) as usize,
										((rate / max) * available_y) as usize,
									);
								}
							};

							if let Some(rate) =
								rate.subjects.iter().find(|s| subject_ids.contains(&s.id))
							{
								process_rate(rate);
							} else if let Some(rate) =
								rate.subjects.iter().find(|s| &s.name == subject_name)
							{
								process_rate(rate);
							}
						}
						points.insert(subject.clone(), temp);
					}
				}

				let ctx = seed::canvas_context_2d(&model.graphs_canvas.get().unwrap());

				ctx.translate(0.0 + margin, available_y + margin).unwrap();
				ctx.scale(1.0, -1.0).unwrap();

				if model.show_grid {
					if model.dark_theme {
						ctx.set_stroke_style(&seed::prelude::JsValue::from_str("#333333"));
					} else {
						ctx.set_stroke_style(&seed::prelude::JsValue::from_str("#DDDDDD"));
					}

					let mut x = x_spacing;
					while x <= available_x {
						ctx.begin_path();
						ctx.move_to(x, 0.0);
						ctx.line_to(x, available_y);
						ctx.stroke();

						x += x_spacing;
					}

					let mut y = 0.25 * available_y;
					while y <= available_y {
						ctx.begin_path();
						ctx.move_to(0.0, y);
						ctx.line_to(available_x, y);
						ctx.stroke();

						y += 0.25 * available_y;
					}
				}

				if model.dark_theme {
					ctx.set_stroke_style(&seed::prelude::JsValue::from_str("#FFFFFF"));
				} else {
					ctx.set_stroke_style(&seed::prelude::JsValue::from_str("#000000"));
				}

				ctx.begin_path();
				ctx.move_to(0.0, 0.0);
				ctx.line_to(available_x, 0.0);
				ctx.stroke();

				ctx.begin_path();
				ctx.move_to(0.0, 0.0);
				ctx.line_to(0.0, available_y);
				ctx.stroke();

				if model.dark_theme {
					ctx.set_fill_style(&seed::prelude::JsValue::from_str("#FFFFFF"));
				} else {
					ctx.set_fill_style(&seed::prelude::JsValue::from_str("#000000"));
				}
				ctx.scale(1.0, -1.0).unwrap();
				let text = format!(
					"{}",
					start_date.format(&crate::locale::get_simple(&model.locale, "date-format"))
				);
				ctx.fill_text(&text, 0.0, 15.0).ok();

				let text = format!(
					"{}",
					end_date.format(&crate::locale::get_simple(&model.locale, "date-format"))
				);
				let text_size = ctx.measure_text(&text).unwrap();
				ctx.fill_text(&text, available_x - 0.0 - text_size.width(), 15.0)
					.ok();
				if available_x >= 3.5 * text_size.width() && (end_date - start_date).num_days() > 1
				{
					let text = format!(
						"{}",
						(start_date + (end_date - start_date) / 2)
							.format(&crate::locale::get_simple(&model.locale, "date-format"))
					);
					let text_size = ctx.measure_text(&text).unwrap();
					ctx.fill_text(&text, available_x / 2.0 - text_size.width() / 2.0, 15.0)
						.ok();
				}
				ctx.scale(1.0, -1.0).unwrap();

				for (subject, data) in &points {
					ctx.begin_path();
					ctx.set_stroke_style(&seed::prelude::JsValue::from_str(&subject.color));
					for (x, y) in data {
						ctx.line_to(*x as f64, *y as f64);
					}
					ctx.stroke();
				}

				if model.show_points {
					for (subject, data) in &points {
						ctx.set_fill_style(&seed::prelude::JsValue::from_str(&subject.color));
						ctx.set_stroke_style(&seed::prelude::JsValue::from_str(&subject.color));
						for (x, y) in data {
							ctx.begin_path();
							ctx.ellipse(
								*x as f64,
								*y as f64,
								3.0,
								3.0,
								0.0,
								0.0,
								2.0 * std::f64::consts::PI,
							)
							.ok();
							ctx.fill();
						}
					}
				}
			}

			if model.do_render_graphics {
				orders
					.after_next_render(|_| crate::messages::Message::Graphs(Message::UpdateGraph));
			}
		}
		Message::SetSubjectValue { name, value } => {
			if let Some((_, old_value)) = model
				.historical_subjects
				.iter_mut()
				.find(|(subject_name, _)| **subject_name == name)
			{
				old_value.checked = value;
			}
		}
		Message::SetAllSubjects(value) => {
			for name in model.historical_subjects.keys() {
				orders.send_msg(crate::messages::Message::Graphs(Message::SetSubjectValue {
					name: name.clone(),
					value,
				}));
			}
		}
		Message::SetStart(value) => {
			model.graph_start = match value {
				Some(value) => Some(chrono::NaiveDate::parse_from_str(&value, "%Y-%m-%d").unwrap()),
				None => None,
			};
		}
		Message::SetEnd(value) => {
			model.graph_end = match value {
				Some(value) => Some(chrono::NaiveDate::parse_from_str(&value, "%Y-%m-%d").unwrap()),
				None => None,
			};
		}
		Message::SetShowPoints(value) => {
			model.show_points = value;
		}
		Message::SetShowGrid(value) => {
			model.show_grid = value;
		}
	}
}
