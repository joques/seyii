use crate::messenger;

#[derive(Debug)]
pub struct MessageBuffer<'a> {
	buff: Vec<&'a messenger::SimpleMessage<'a>>,
}

pub fn create_buffer<'a>() -> MessageBuffer<'a> {
	MessageBuffer {
		buff: Vec::new()
	}
}

pub fn add_to_buffer<'a>(mb: &mut MessageBuffer<'a>, msg: &'a messenger::SimpleMessage<'a>) {
	mb.buff.push(msg);
}

pub fn find_message<'a>(mb: &'a MessageBuffer<'a>, message_id: u32) -> Result<&'a messenger::SimpleMessage<'a>, &str> {
	let msg_iter = mb.buff.iter();

	for msg in msg_iter {
		if msg.mid == message_id {
			return Ok(msg);
		} else {
			continue;
		}
	}

	let err_msg: String = format!("Simple Message not found with id {}", message_id);
	return Err("Simple Message not found!")
}