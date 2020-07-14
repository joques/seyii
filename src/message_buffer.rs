mod messenger;

#[derive(Debug)]
struct MessageBuffer {
	buff: Vec<messenger::SimpleMessage>,
}

impl MessageBuffer {
	fn new() -> MessageBuffer {
		MessageBuffer {
			buff: Vec::new()
		}
	}

	fn add_to_buffer(&mut self, msg: messenger::SimpleMessage) {
		self.buff.push(msg);
	}

	fn find_message(&self, messsage_id: u32) -> Result<messenger::SimpleMessage, &str> {
		let msg_iter = self.buff.iter();

		for msg in msg_iter {
			if messsage_id == msg.mid {
				return Ok(msg);
			} else {
				continue;
			}
		}
		return Err("Message not found");
	} 
}