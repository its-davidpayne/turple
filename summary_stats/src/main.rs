use std::collections::HashMap;

fn calculate_mean(numbers: &[i32]) -> f64 {
    f64::from(numbers.iter().sum::<i32>()) / f64::from(numbers.len() as i32)
}

fn calculate_median(numbers: &mut [i32]) -> f64 {
    numbers.sort();
    let median = if numbers.len() % 2 == 0 { // numbers has an even length
        let mid = numbers.len() / 2;
        f64::from((numbers[mid] as i32 + numbers[mid -1] as i32) / 2)
    } else { // numbers has an odd length
    let mid = numbers.len() / 2;
    f64::from(numbers[mid])
    };
    median
}

fn calculate_mode(numbers: &[i32]) -> i32 {
    let mut map = HashMap::new();
    for &value in numbers {
        *map.entry(value).or_insert(0) += 1;
    }
    // I originally used a long inline stmt copied from SE for this bit, 
    // but that felt like cheating. This is probably less idiomatic Rust
    // but at least it's all mine.
    let mut most_freq: i32 = 0;
    let mut count_most_freq: i32 = 0;
    for (key, value) in map {
        if value > count_most_freq {
            most_freq = key;
            count_most_freq = value;
        }
    }
    most_freq
}

fn main() {
    let mut numbers = [4, 8, 15, 16, 23, 42, 4, 4, 1, 2, 3, 99];
    println!("The input array is: \n{:#?}", numbers);

    println!("The mean is {}", calculate_mean(&numbers));

    println!("The median is {}", calculate_median(&mut numbers));

    println!("The mode is {}", calculate_mode(&numbers));
}
