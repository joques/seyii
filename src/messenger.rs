#[derive(Debug)]
pub struct SimpleMessage<'a> {
	pub mid: u32,
	pub sender: &'a str,
	pub topic: &'a str,
	pub content: &'a str,
}

pub fn create_simple_message<'a>(message_count: u32, sender: &'a str, topic: &'a str, content: &'a str) -> SimpleMessage<'a> {
	SimpleMessage {
		mid: message_count,
		sender,
		topic,
		content,
	}
}