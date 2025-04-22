use skip_rs::SkipList;
use std::io::{self, Write};

fn main() {
    let mut skip_list = SkipList::new();
    println!("Welcome to the SkipList<isize, isize> program!");
    loop {
        println!();
        println!("Choose an option:");
        println!("1. Insert a key-value pair");
        println!("2. Pop a key-value pair");
        println!("3. Pop a key-value pair by index");
        println!("4. Peek the key-value pair at a given index");
        println!("5. Clear the skip list");
        println!("6. Print the skip list");
        println!("7. Exit");
        println!();
        println!("Enter your choice:");
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice = choice.trim().parse::<u32>();

        match choice {
            Ok(1) => {
                print!("\nEnter a value to insert: ");
                io::stdout().flush().expect("Failed to flush stdout");
                let mut value = String::new();
                io::stdin()
                    .read_line(&mut value)
                    .expect("Failed to read input");
                if let Ok(value) = value.trim().parse::<isize>() {
                    print!("\nEnter a key for this value: ");
                    io::stdout().flush().expect("Failed to flush stdout");
                    let mut key = String::new();
                    io::stdin()
                        .read_line(&mut key)
                        .expect("Failed to read input");
                    if let Ok(key) = key.trim().parse::<isize>() {
                        skip_list.insert(key, value);
                        println!("Inserted key-value pair: {} -> {}", key, value);
                    } else {
                        println!("Invalid key. It should be an integer.");
                    }
                } else {
                    println!("Invalid value. It should be an integer.");
                }
            }
            Ok(2) => {
                print!("\nEnter the key to remove: ");
                io::stdout().flush().expect("Failed to flush stdout");
                let mut key = String::new();
                io::stdin()
                    .read_line(&mut key)
                    .expect("Failed to read input");
                if let Ok(key) = key.trim().parse::<isize>() {
                    match skip_list.pop(key) {
                        Ok((key, value)) => {
                            println!("Popped key-value pair: {} -> {}", key, value)
                        }
                        Err(_) => println!("Key not found."),
                    };
                } else {
                    println!("Invalid key. It should be an integer.");
                }
            }
            Ok(3) => {
                print!("\nEnter an index to remove: ");
                io::stdout().flush().expect("Failed to flush stdout");
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to read input");
                if let Ok(index) = index.trim().parse::<usize>() {
                    match skip_list.pop_at_index(index) {
                        Ok((key, value)) => {
                            println!(
                                "Popped key-value pair at index {}: {} -> {}",
                                index, key, value
                            )
                        }
                        Err(_) => println!("Index out of bounds."),
                    }
                } else {
                    println!("Invalid index. It should be a non-negative integer.");
                }
            }
            Ok(4) => {
                print!("\nEnter an index to print: ");
                io::stdout().flush().expect("Failed to flush stdout");
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to read input");
                if let Ok(index) = index.trim().parse::<usize>() {
                    match skip_list.peek_at_index(index) {
                        Ok((key, value)) => {
                            println!("Key-value pair at index {}: {} -> {}", index, key, value)
                        }
                        Err(_) => println!("Index out of bounds."),
                    }
                } else {
                    println!("Invalid index. It should be a non-negative integer.");
                }
            }
            Ok(5) => {
                print!("Clearing the skip list...");
                skip_list.clear();
                println!("Done.");
            }
            Ok(6) => {
                println!("Skip list contents:\n{}", skip_list);
            }
            Ok(7) => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}
