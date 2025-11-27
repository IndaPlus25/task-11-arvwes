#[cfg(test)]
mod tests;
use rand::seq::SliceRandom;



fn main() {
    let list = vec![64, 25, 12, 22, 11];
    let sorted_list = bogosort(list);
    println!("{:?}", sorted_list);
}

pub fn selection_sort(mut list: Vec<usize>) -> Vec<usize> {
    let n = list.len();

    for j in 0..n - 1 {
        let mut smallest = j;
        for i in j + 1..n {
            if list[i] < list[smallest] {
                smallest = i;
            }
        }
        list.swap(j, smallest);
    }

    return list;
}

pub fn insertion_sort(mut list: Vec<usize>) -> Vec<usize> {
    for j in 1..list.len() {
        let key = list[j];
        let mut i: isize = j as isize - 1;
        while i >= 0 && list[i as usize] > key {
            list[(i + 1) as usize] = list[i as usize];
            i -= 1;
        }
        list[(i + 1) as usize] = key;
    }
    list
}

pub fn merge_sort(mut list: Vec<usize>, first: usize, last: usize) -> Vec<usize> {
    if first < last {
        let midpoint: usize = first + (last - first) / 2;

        list = merge_sort(list, first, midpoint);
        list =merge_sort(list, midpoint + 1, last);

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
            i+= 1;
        } else {
            list[k] = right[j];
            j += 1;

        }
    }
    list
}

pub fn bogosort(mut list: Vec<usize>) -> Vec<usize> {
    while !list.is_sorted() {
        list.shuffle(&mut rand::rng());
    }
    list
}