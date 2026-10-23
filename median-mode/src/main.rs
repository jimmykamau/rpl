use std::collections::HashMap;

fn get_median(numbers: &[i32]) -> i32 {
    let mut numbers_vec = numbers.to_vec();
    numbers_vec.sort();
    numbers_vec[numbers.len() / 2]
}

fn get_mode(numbers: &[i32]) -> i32 {
    let mut map = HashMap::new();
    let mut most_often: (i32, i32) = (numbers[0], 1);
    for num in numbers {
        let count = map.entry(num).or_insert(0);
        *count += 1;
        if most_often.1 < *count {
            most_often = (*num, *count);
        }
    }
    most_often.0
}

fn main() {
    let my_arr = [4, 6, 8, 7, -9, 8, 10, 6, 8];
    let med = get_median(&my_arr);
    assert_eq!(med, 7);
    let mode = get_mode(&my_arr);
    assert_eq!(mode, 8);
}
