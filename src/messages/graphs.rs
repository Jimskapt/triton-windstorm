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
}

pub fn update(
	message: Message,
	model: &mut crate::model::Model,
	orders: &mut impl seed::prelude::Orders<crate::messages::Message>,
) {
	match message {
		Message::ComputeHistoricalSubjects => {
			let mut temp_subjects = model.subjects.clone();
			for (_, record) in &model.records {
				for subject in &record.subjects {
					if let None = temp_subjects.get(&subject.id) {
						if let None = temp_subjects.iter().find(|(_, s)| s.name == subject.name) {
							temp_subjects.insert(subject.id.clone(), subject.clone());
						}
					}
				}
			}
			// TODO : cleanup of empty names ?

			model.historical_subjects = temp_subjects
				.values()
				.map(|subject| {
					let mut color = String::from("#");
					for _ in 0..6 {
						color += rand::seq::IteratorRandom::choose(
							ALPHABET.iter(),
							&mut rand::thread_rng(),
						)
						.unwrap();
					}

					(
						subject.name.clone(),
						crate::model::HistoricalSubject {
							checked: true,
							color,
						},
					)
				})
				.collect();
		}
		Message::UpdateGraph => {
			if let Some(canvas) = model.graphs_canvas.get() {
				canvas.set_width(canvas.client_width() as u32);
				canvas.set_height(canvas.client_height() as u32);

				let margin = 30.0;
				let available_x = canvas.client_width() as f64 - 2.0 * margin;
				let available_y = canvas.client_height() as f64 - 2.0 * margin;
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
				let date_range = model.graph_end.unwrap_or_else(|| {
					chrono::NaiveDate::parse_from_str(
						model.records.keys().last().unwrap_or(&today_str),
						"%Y-%m-%d",
					)
					.unwrap()
				}) - start_date;
				let x_spacing = f64::abs(available_x as f64 / (date_range.num_days() as f64));
				let mut points = std::collections::BTreeMap::new();
				for (subject_name, subject) in &model.historical_subjects {
					if subject.checked {
						let mut temp = std::collections::BTreeMap::new();
						for (day, rate) in &model.records {
							if let Some(rate) =
								rate.subjects.iter().find(|s| &s.name == subject_name)
							{
								let max = rate.max;
								if let Some(rate) = rate.value {
									temp.insert(
										((chrono::NaiveDate::parse_from_str(&day, "%Y-%m-%d")
											.unwrap() - start_date)
											.num_days() as f64 * x_spacing) as usize,
										((rate / max) * available_y) as usize,
									);
								}
							} // else find id ?
						}
						points.insert(subject.clone(), temp);
					}
				}

				let ctx = seed::canvas_context_2d(&model.graphs_canvas.get().unwrap());

				ctx.translate(0.0 + margin, available_y + margin).unwrap();
				ctx.scale(1.0, -1.0).unwrap();

				ctx.set_stroke_style(&seed::prelude::JsValue::from_str("#FFFFFF"));

				ctx.begin_path();
				ctx.move_to(0.0, 0.0);
				ctx.line_to(available_x, 0.0);
				ctx.stroke();

				ctx.begin_path();
				ctx.move_to(0.0, 0.0);
				ctx.line_to(0.0, available_y);
				ctx.stroke();

				for (subject, data) in points {
					ctx.begin_path();
					ctx.set_stroke_style(&seed::prelude::JsValue::from_str(&subject.color));
					for (x, y) in data {
						ctx.line_to(x as f64, y as f64);
					}
					ctx.stroke();
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
	}
}
