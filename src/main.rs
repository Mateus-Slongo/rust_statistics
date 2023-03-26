use std::collections::HashMap;
use std::hash::Hash;
use std::num;

fn mean(arr: &[f32]) -> f32 {
    arr.iter().sum::<f32>() / arr.len() as f32
}

fn variance(arr: &[f32]) -> f32 {
    let mean = mean(arr);
    arr.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / arr.len() as f32
}

fn standard_deviation(arr: &[f32]) -> f32 {
    variance(arr).sqrt()
}

fn median(arr: &[f32]) -> f32 {
    let mut sorted = arr.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) / 2.0
    } else {
        sorted[mid]
    }
}

pub fn mode<T: Copy + Into<f32>>(arr: &[T]) -> f32 {
    let mut counts = HashMap::new();
    let mut max_count = 0;
    let mut mode: f32 = 0.0;

    // Determine the maximum count and mode value
    for &val in arr {
        let count = counts.entry(val.into() as i32).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            mode = val.into() as f32;
        }
    }

    mode
}

fn main() {
    let arr = [1.0, 2.0, 5.0, 6.0, 10.0, 3.0, 5.0];
    println!("Mean: {}", mean(&arr[..]));
    println!("Median: {}", median(&arr[..]));
    let mode = mode(&arr[..]);
    println!("Mode: {}", mode);
    println!("Variance: {}", variance(&arr[..]));
    println!("Standard Deviation: {}", standard_deviation(&arr[..]));
}
