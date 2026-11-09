use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind};

// Read
fn load_items() -> Result<Vec<String>, io::Error> {
    let file = File::open("chest.txt")?;
    let reader = BufReader::new(file);
    let mut items = Vec::new();

    for line in reader.lines() {
        let item = line?;
        if !item.trim().is_empty() {
            items.push(item.trim().to_string());
        }
    }

    Ok(items)
}
 // vec::get
fn check_slot(items: &[String], index: usize) -> Option<&String> {
    items.get(index)
}

//  Panic!
fn trigger_impossible_state() {
    panic!("Impossible game state: chest exists in the void!");
}

fn main() {
    // load_items-funktio 
    let items = match load_items() {
        Ok(loaded) => loaded,
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                println!("No chest file found; using the standard supplies.");
                vec![
                    String::from("Rope"),
                    String::from("Torch"),
                    String::from("Potion"),
                ]
            } else {
                eprintln!("Error reading chest file: {}", error);
                return;
            }
        }
    };

    println!("The chest contains:");
    for item in &items {
        println!("- {}", item);
    }

    // Safe check 
    let valid_index = 1;
    match check_slot(&items, valid_index) {
        Some(item) => println!("Slot {} contains {}.", valid_index, item),
        None => println!("There is no item in slot {}.", valid_index),
    }

    let invalid_index = 99;
    match check_slot(&items, invalid_index) {
        Some(item) => println!("Slot {} contains {}.", invalid_index, item),
        None => println!("There is no item in slot {}.", invalid_index),
    }

    trigger_impossible_state();

    // " RUST_BACKTRACE=1 cargo run " tulostaa funktion kutsupolun (stack backtrace),
    // joka auttaa paikantamaan tarkan rivin ja tiedoston, josta panic! käynnistyi.
    // Suora indeksöinti myös rajojen ulkopuolelle aiheuttaa paniikin:
    // let _out_of_bounds = &items[99];
}