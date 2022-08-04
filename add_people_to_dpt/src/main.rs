use itertools::Itertools;
use std::collections::HashMap;

use std::fmt;

struct RecordParseError;
impl fmt::Display for RecordParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse record")
    }
}

fn main() {
    let mut m = HashMap::new();

    for record in [
        "Add Ann to Engineers",
        "Add Peter to Engineers",
        "Add Harry to Cleaners",
    ] {
        match add_record(record, &mut m) {
            Err(e) => println!("Error adding record to storage: {}", e),
            _ => println!("Success!"),
        }
    }

    println!("{:?}", m);

    // Get people form Eng. dpt.
    println!("{:?}", people_in_dpt("Engineers", &mut m));

    // Get all people sorted.
    println!("{:?}", all_people_sorted(&mut m));
}

fn add_record(
    record: &str,
    storage: &mut HashMap<String, Vec<String>>,
) -> Result<(), RecordParseError> {
    let mut it = record.split_whitespace();

    it.next();

    let name = match it.next() {
        Some(value) => value,
        None => return Err(RecordParseError),
    };

    it.next();

    let dept = match it.next() {
        Some(value) => value,
        None => return Err(RecordParseError),
    };

    storage
        .entry(dept.to_string())
        .or_insert(Vec::new())
        .push(name.to_string());

    Ok(())
}

fn people_in_dpt(dpt: &str, storage: &mut HashMap<String, Vec<String>>) -> Vec<String> {
    match storage.get(&dpt.to_string()) {
        Some(v) => v.to_vec(),
        _ => Vec::new(),
    }
}

fn all_people_sorted(storage: &mut HashMap<String, Vec<String>>) -> Vec<String> {
    Itertools::sorted(
        storage
            .iter()
            .fold(
                Vec::<String>::with_capacity(storage.len()),
                |acc: Vec<String>, (_, value)| [acc, value.to_vec()].concat(),
            )
            .into_iter(),
    )
    .collect::<Vec<String>>()
}
