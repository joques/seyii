mod messenger;
mod message_buffer;
mod message_storage;

fn main() {
    println!("Welcome to SeyiI!");

    println!("Creating a simple message...");

    let mut simple_message_count = 1;
    let m: messenger::SimpleMessage = messenger::create_simple_message(simple_message_count, "jose", "mom", "First message for MOM testing...");
    println!("here is the created message {:?}", m);

    let mut mb: message_buffer::MessageBuffer = message_buffer::create_buffer();
    println!(" Just created a message buffer{:?}", mb);

    message_buffer::add_to_buffer(&mut mb, &m);
    println!("revisiting the message buffer {:?}", mb);

    println!("checking borrowing rule for message m {:?}", m);

    simple_message_count += 1;

    let m1: messenger::SimpleMessage = messenger::create_simple_message(simple_message_count, "jose", "mom", "Second message for MOM testing...");
    println!("here is the created message {:?}", m1);

    message_buffer::add_to_buffer(&mut mb, &m1);
    println!("revisiting the message buffer {:?}", mb);

    let _res = match message_buffer::find_message(&mb, 2) {
    	Some(m3) => println!("here is the retrieved message {:?}", m3),
    	None => println!("No such message"),
    };

    let mut store = message_storage::Storage::create();
    println!("here is the new message store {:?}", store);

    let _res2 = match store.store_message(&m1) {
    	Err(errstr) => println!("An error occurred while storing message {:?}. Error message reads {}", m1, errstr),
    	Ok(resmsg) => println!("The message storage was ok {}", resmsg),
    };
}
