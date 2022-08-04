fn main() {
    println!(
        "{}",
        to_pig_latin(&String::from("sdf sA asdf sdfasdf fghfg wer iosdf"))
    )
}

fn to_pig_latin(s: &String) -> String {
    s.split_whitespace()
        .map(|word| {
            let c = word.to_string().chars().nth(0).unwrap();
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => String::from(word.to_string() + "-hay"),
                _ => {
                    let mut s = String::from(word) + "-";
                    s.remove(0);
                    s.push(c);
                    s + "ay"
                }
            }
        })
        .collect::<Vec<String>>()
        .join(", ")
}
