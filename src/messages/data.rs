pub enum Message {
	SetPretty(bool),
}

pub fn update(
	message: Message,
	model: &mut crate::model::Model,
	_orders: &mut impl seed::prelude::Orders<crate::messages::Message>,
) {
	match message {
		Message::SetPretty(val) => model.pretty_export = val,
	}
}
