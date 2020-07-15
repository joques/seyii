/* message_buffer.rs */ 

use crate::messenger;

#[derive(Debug)]
pub struct MessageBuffer<'a> {
	buff: Vec<&'a messenger::SimpleMessage<'a>>,
}

impl<'a> MessageBuffer<'a> {
	pub fn create() -> Self {
		return MessageBuffer {
			buff: Vec::new()
		}
	}

	pub fn add_to_buffer(&mut self, msg: &'a messenger::SimpleMessage<'a>) -> Result<&str, &str> {
		self.buff.push(msg);
		return Ok("New mesage added")
	}

	pub fn find_message(&self, message_id: u32) -> Option<&'a messenger::SimpleMessage<'a>> {
		let msg_iter = self.buff.iter();

		for  msg in msg_iter {
			if msg.mid == message_id {
				return Some(msg);
			} else {
				continue;
			}
		}
		
		return None
	}
} 

// pub fn create_buffer<'a>() -> MessageBuffer<'a> {
// 	MessageBuffer {
// 		buff: Vec::new()
// 	}
// }

// pub fn add_to_buffer<'a>(mb: &mut MessageBuffer<'a>, msg: &'a messenger::SimpleMessage<'a>) {
// 	mb.buff.push(msg);
// }

// pub fn find_message<'a>(mb: &'a MessageBuffer<'a>, message_id: u32) -> Option<&'a messenger::SimpleMessage<'a>> {
// 	let msg_iter = mb.buff.iter();

// 	for msg in msg_iter {
// 		if msg.mid == message_id {
// 			return Some(msg);
// 		} else {
// 			continue;
// 		}
// 	}
	
// 	return None
// }