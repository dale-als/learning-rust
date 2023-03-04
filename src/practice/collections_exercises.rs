use std::collections::HashMap;

fn main() {
    let int_list = [1, 1, 2, 3, 4, 5, 4, 6, 8, 4];
    let mut vector: Vec<i32> = int_list.to_vec();

    // vector.sort(); -- usable if vector elements type implement cmp::Ord trait

    for int in &vector {
        // println!("{int}");
    }

    // println!("Length: {}", vector.len());
    // println!("Median: {}", find_median(&vector));
    // println!("Mode: {}", find_mode(&vector));

    println!("{}", pig_latin("first apple"));
}

fn find_median(int_vector: &Vec<i32>) -> i32 {
    let mut vector_copy = int_vector.clone();

    vector_copy.sort();

    let middle: f32 = vector_copy.len() as f32 / 2.0;

    // rounding tests
    // let ceil = middle.ceil() as usize;
    let floor = middle.floor() as usize;
    let middle_value = &vector_copy[floor];

    *middle_value
}

fn find_mode(int_vector: &Vec<i32>) -> i32 {
    if int_vector.len() == 0 {
        return 0;
    }

    let mut max_count = 0;
    let mut mode = 0;
    let mut occurences = HashMap::new();

    for int in int_vector {
        let count = occurences.entry(int).or_insert(0);
        *count += 1;
    }

    for (_key, value) in &occurences {
        if *value > max_count {
            max_count = *value;
        }
    }

    for (key, value) in occurences {
        if value == max_count {
            mode = *key;
            break;
        }
    }

    mode
}

fn pig_latin(words: &str) -> String {
    let mut result = String::from("");
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];

    for (index, word) in words.split_whitespace().enumerate() {
        match word.chars().next() {
            Some(char) => {
                if vowels.contains(&char) {
                    let pigged = format!("{word}-hay");
                    result.push_str(&pigged[..]);
                } else {
                    let pigged = format!("{}-{}ay", &word[char.len_utf8()..], char);
                    result.push_str(&pigged[..]);
                }
            }
            _ => (),
        }

        if index < word.len() {
            result.push_str(" ");
        }
    }

    result
}
