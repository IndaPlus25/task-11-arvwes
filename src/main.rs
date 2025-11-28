#[cfg(test)]
mod tests;

use std::io::{self, Write};
use rand::Rng;
use rand::seq::SliceRandom;

fn main() {
    
    let mut rng = rand::thread_rng();

    loop {
        println!("\n=== Sorting Visualizer ===");
        println!("1. Selection Sort");
        println!("2. Insertion Sort");
        println!("3. Merge Sort");
        println!("4. Bogosort");
        println!("5. Exit");
        print!("Choose algorithm: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        let mut list: Vec<usize> = Vec::new();
        for _ in 0..20 {
            list.push(rng.gen_range(1..=100));
        }

        match choice.trim() {
            "1" => {
                selection_sort(list.clone());
            }
            "2" => {
                insertion_sort(list.clone());
            }
            "3" => {
                let n = list.len();
                merge_sort(list.clone(), 0, n - 1);
            }
            "4" => {
                bogosort(list.clone());
            }
            "5" => break,
            _ => println!("Invalid choice"),
        }
    }
}

fn print_bars(arr: &[usize], step: usize) {
    println!("\nStep {}: {:?}", step, arr);
    for &num in arr {
        print!("-");
        for _ in 0..(num / 4) {
            print!("-");
        }
        println!();
    }
    std::thread::sleep(std::time::Duration::from_millis(300));
}

pub fn selection_sort(mut list: Vec<usize>) -> Vec<usize> {
    let n = list.len();
    let mut step = 0;

    for j in 0..n - 1 {
        print_bars(&list, step);
        step += 1;
        let mut smallest = j;
        for i in j + 1..n {
            if list[i] < list[smallest] {
                smallest = i;
            }
        }
        
        list.swap(j, smallest);
    }
    print_bars(&list, step);
    return list;
}

pub fn insertion_sort(mut list: Vec<usize>) -> Vec<usize> {
    let mut step = 0;
    for j in 1..list.len() {
        print_bars(&list, step);
        step += 1;
        let key = list[j];
        let mut i: isize = j as isize - 1;
        while i >= 0 && list[i as usize] > key {
            list[(i + 1) as usize] = list[i as usize];
            i -= 1;
        }
        list[(i + 1) as usize] = key;
    }
    print_bars(&list, step);
    list
}

pub fn merge_sort(mut list: Vec<usize>, first: usize, last: usize) -> Vec<usize> {
    if first < last {
        let midpoint: usize = first + (last - first) / 2;

        list = merge_sort(list, first, midpoint);
        list = merge_sort(list, midpoint + 1, last);

        list = merge(list, first, midpoint, last);
    }

    list
}

fn merge(mut list: Vec<usize>, first: usize, midpoint: usize, last: usize) -> Vec<usize> {
    let n_left = midpoint - first + 1;
    let n_right = last - midpoint;
    // Create temp arrays
    let mut left = Vec::with_capacity(n_left + 1);
    let mut right = Vec::with_capacity(n_right + 1);
    // Copy data to temp arrays left[] and right[]
    for i in 0..n_left {
        left.push(list[first + i]);
    }
    for i in 0..n_right {
        right.push(list[midpoint + 1 + i]);
    }
    // Add sentinels for easier comparison
    left.push(usize::MAX);
    right.push(usize::MAX);

    // Merge the temp arrays back into list[first..last]
    let mut i = 0;
    let mut j = 0;

    for k in first..=last {
        if left[i] <= right[j] {
            list[k] = left[i];
            i += 1;
        } else {
            list[k] = right[j];
            j += 1;
        }
        print_bars(&list, k);
    }
    list
}

pub fn bogosort(mut list: Vec<usize>) -> Vec<usize> {
    let mut step = 0;
    while !list.is_sorted() {
        print_bars(&list, step);
        step += 1;
        list.shuffle(&mut rand::rng());
    }
    print_bars(&list, step);
    list
}
