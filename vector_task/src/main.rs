use std::collections::HashMap;

fn main() {
    let mut my_vec = vec![1, 2, 3, 3, 4, 5, 1, 3, 4, 10];
    let my_vec_len = my_vec.len();

    let sum: i32 = my_vec.iter().sum();
    let mean: f32 = sum as f32 / my_vec_len as f32;
    println!("Mean: {}", mean);

    my_vec.sort();

    let median: f32 = match my_vec_len {
        x if x % 2 != 0 => my_vec[my_vec_len / 2] as f32,
        _ => my_vec[(my_vec_len - 1) / 2] as f32 + my_vec[my_vec_len / 2] as f32,
    };
    println!("Median: {}", median);

    let mut map = HashMap::new();
    for i in my_vec {
        let cnt = map.entry(i).or_insert(0);
        *cnt += 1;
    }

    let mut mod_vec = 0;
    let mut max_used = 0;
    for (k, v) in map {
        if v > max_used {
            mod_vec = k;
            max_used = v;
        }
    }
    println!("Mod: {}", mod_vec);
}
