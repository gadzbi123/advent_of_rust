use read_file::read_file;
use std::collections::{HashMap, HashSet};
fn task_14_1() {
    let lines = read_file("./data_files/file14.txt");
    let mut polymer = lines.first().unwrap().to_string();
    dbg!(&polymer);
    let mut modification: HashMap<(char, char), char> = HashMap::new();
    for line in lines.iter().skip(2) {
        let (key, value) = line.split_once(" -> ").unwrap();
        let mut char_iter = key.chars();
        let new_key = (char_iter.next().unwrap(), char_iter.next().unwrap());
        modification.insert(new_key, value.chars().next().unwrap());
    }
    dbg!(&modification);
    for i in 0..1 {
        dbg!(i);
        let windows = polymer.as_bytes().windows(2);
        let mut new_polymer = String::new();
        for pair in windows {
            let left_key = pair[0] as char;
            let right_key = pair[1] as char;
            let new_char = modification.get(&(left_key, right_key)).unwrap();
            new_polymer.push(left_key);
            new_polymer.push(*new_char);
        }
        new_polymer.push(polymer.chars().last().unwrap());
        polymer = new_polymer;
    }
    let mut min = usize::MAX;
    let mut max = usize::MIN;
    for unique_char in polymer.chars()
    /*.unique()*/
    {
        let count = polymer.chars().filter(|&ch| unique_char == ch).count();
        min = min.min(count);
        max = max.max(count);
    }
    println!("Task 14.1 = {}, polymer: {}", max - min, polymer);
    dbg!(polymer.len());
}
fn task_14_2_unfinished() {
    let lines = read_file("./data_files/file14.txt");
    let polymer = lines.first().unwrap().to_string();

    let mut modification: HashMap<(char, char), char> = HashMap::new();
    let mut unique_chars: HashSet<char> = HashSet::new();

    for line in lines.iter().skip(2) {
        let (key, value) = line.split_once(" -> ").unwrap();
        let mut char_iter = key.chars();
        let new_key = (char_iter.next().unwrap(), char_iter.next().unwrap());
        let char_value = value.chars().next().unwrap();
        modification.insert(new_key, char_value);
        unique_chars.insert(char_value);
    }
    // dbg!(&modification);
    // NCNBCHB
    let mut polimer_counter: HashMap<(char, char), usize> = HashMap::new();

    let windows = polymer.as_bytes().windows(2);
    for pair in windows {
        let left_key = pair[0] as char;
        let right_key = pair[1] as char;
        let char_pair = &(left_key, right_key);
        polimer_counter.insert(
            *char_pair,
            polimer_counter
                .get(char_pair)
                .unwrap_or(&0)
                .saturating_add(1),
        );
    }
    for i in 0..10 {
        for (char_pair, count) in polimer_counter.clone() {
            let left_key = char_pair.0;
            let right_key = char_pair.1;
            polimer_counter.insert(
                char_pair,
                polimer_counter
                    .get(&char_pair)
                    .unwrap_or(&0)
                    .saturating_sub(1),
            );
            let new_char = *modification.get(&(left_key, right_key)).unwrap();
            let left_pair = (left_key, new_char);
            polimer_counter.insert(
                left_pair,
                *polimer_counter.get(&left_pair).unwrap_or(&0) + 1,
            );
            let right_pair = (new_char, right_key);
            polimer_counter.insert(
                right_pair,
                *polimer_counter.get(&right_pair).unwrap_or(&0) + 1,
            );
            // dbg!(char_pair, left_pair, right_pair);
        }
    }
    let ending_char = polymer.as_bytes().get(polymer.len() - 1).unwrap();
    let mut min = usize::MAX;
    let mut max = usize::MIN;
    for c in unique_chars {
        let count = polimer_counter
            .iter()
            .filter(|((left_key, right_key), value)| *left_key == c)
            .fold(0, |acc, (key, val)| acc + val);
        min = min.min(count);
        max = max.max(count);
    }
    dbg!(max, min);
    dbg!(polimer_counter);
}
