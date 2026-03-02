use std::io;

fn main() {
    println!("\nEnter the quantity of IDs to generate (max 256)\n");

    let mut qty = String::new();

    io::stdin()
        .read_line(&mut qty)
        .expect("Failed to read request");
}
