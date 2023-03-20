use std::collections::HashMap;

pub fn median(numbers_: &Vec<i32>) -> f32 {
    let mut numbers = numbers_.clone();
    numbers.sort();
    let mid = numbers.len() / 2;

    if mid % 2 == 0 {
        return (numbers[mid] + numbers[mid + 1]) as f32 / 2.;
    }

    numbers[mid + 1] as f32
}

pub fn mode(numbers: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for &value in numbers {
        *map.entry(value).or_insert(0) += 1;
    }

    map
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}

