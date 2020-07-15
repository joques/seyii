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

	//add a simple message to the buffer so it can be retrieved when the message is lost 
	pub fn add_to_buffer(&mut self, msg: &'a messenger::SimpleMessage<'a>) -> Result<&str, &str> {
		self.buff.push(msg);
		return Ok("New mesage added")
	}

	// find a message from the buffer in order to share it wit iots in case of message loss
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