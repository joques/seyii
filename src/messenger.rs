#[derive(Debug)]
pub struct SimpleMessage<'a> {
	mid: u32,
	sender: &'a str,
	topic: &'a str,
	content: &'a str,
}

pub fn create_simple_message<'a>(message_count: u32, sender: &'a str, topic: &'a str, content: &'a str) -> SimpleMessage<'a> {
	SimpleMessage {
		mid: message_count,
		sender,
		topic,
		content,
	}
}