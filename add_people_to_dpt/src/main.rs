use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;

fn main() {}

fn add_record(
    record: &str,
    storage: &mut HashMap<String, Vec<String>>,
) -> Result<(), Box<dyn Error>> {
    let mut it = record.split_whitespace();

    it.next();

    let name = match it.next() {
        Some(value) => value,
        None => Err("fail to parse name")?,
    };

    it.next();

    let dept = match it.next() {
        Some(value) => value,
        None => Err("fail to parse dept")?,
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

fn all_people_sorted(storage: &HashMap<String, Vec<String>>) -> Vec<String> {
    storage
        .iter()
        .flat_map(|(_, value)| value.iter().cloned())
        .sorted()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_case() {
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

        // Create hash map with depts and people
        assert_eq!(
            HashMap::from([
                (
                    String::from("Engineers"),
                    vec![String::from("Ann"), String::from("Peter")]
                ),
                (String::from("Cleaners"), vec![String::from("Harry")]),
            ]),
            m
        );

        // Get people form Eng. dpt.
        assert_eq!(
            vec![String::from("Ann"), String::from("Peter")],
            people_in_dpt("Engineers", &mut m),
        );

        // Get all people sorted
        assert_eq!(
            vec![
                String::from("Ann"),
                String::from("Harry"),
                String::from("Peter")
            ],
            all_people_sorted(&m)
        )
    }
}
