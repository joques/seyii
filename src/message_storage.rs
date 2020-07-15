/* message_storage */ 

use crate::messenger;

#[derive(Debug)]
pub struct Storage<'a> {
	idents: Vec<u32>,
	messages: Vec<&'a messenger::SimpleMessage<'a>>, 
}

impl<'a> Storage<'a> {
	pub fn create() -> Self {
		return Storage{
			idents: Vec::new(),
			messages: Vec::new()
		}
	}

	pub fn store_message(&mut self, msg: &'a messenger::SimpleMessage<'a>) -> Result<&str, &str> {
		// First check if the message has already been stored
		if self.idents.iter().any(|&curid| curid == msg.mid) {
			// message exists in the store return an error message
			return Err("Message already in store");
		} else {
			// message not found. Will add it to store and send an ok message
			self.idents.push(msg.mid);
			self.messages.push(msg);
			return Ok("New message added to store");
		}
	}
}