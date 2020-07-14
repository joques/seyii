mod messenger;
mod message_buffer;

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
    	Ok(m3) => println!("here is the retrieved message {:?}", m3),
    	Err(em) => println!("and the error message is {:?}", em),
    };
}
