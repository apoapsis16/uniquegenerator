use std::io;
use uuid::Uuid;

fn main() {
    println!("\nEnter the quantity of IDs to generate (max 256)\n");

    let mut user_request = String::new();

    io::stdin()
        .read_line(&mut user_request)
        .expect("Failed to read request");

    // handle bad (noninteger) input
    let qty: u32 = match user_request.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(),
    };

    println!("You requested {qty} ids\n");

    // create and display test id 
    // let test_id = Uuid::new_v4();
    // println!("Test ID: {test_id}");

    // array of ids and iterator
    let mut generated_ids: Vec<Uuid> = Vec::new();

    for i in 0..qty {
        generated_ids.push(Uuid::new_v4());
    }

    for element in &generated_ids {
        println!("{element} "); 
    }

    println!();

    // read to keep CLI open
    io::stdin().read_line(&mut String::new());
}
