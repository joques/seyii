/* 
message_storage.rs 
*/ 

use std::collections::HashMap;
use crate::messenger;

// should probably change the members in the struct with a hashmap

#[derive(Debug)]
pub struct Storage<'a> {
	store: HashMap<&'a str, Vec<&'a messenger::SimpleMessage<'a>>>
}

impl<'a> Storage<'a> {
	pub fn create() -> Self {
		Self{
			store: HashMap::new(),
		}
	}

	pub fn add_message(&mut self, topic: &'a str, msg: &'a messenger::SimpleMessage) -> Result<&str, &str> {
		// first check if the topic has been stored before
		match self.store.get_mut(topic) {
			None => {
				// the topic has never been inserted
				let mut new_topic_store = Vec::new();
				new_topic_store.push(msg);
				self.store.insert(topic, new_topic_store);
				Ok("Message added for new topic")
			}
			Some(messages) => {
				if messages.iter().any(|cur_msg| cur_msg == msg) {
					// the message has already been stored
					Err("This message has already been stored for the topic")
				} else {
					messages.push(msg);
					Ok("New message added for topic")
				}
			}
		}
	}
}