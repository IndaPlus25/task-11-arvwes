use std::intrinsics::floorf128;

fn main() {
    println!("Hello, world!");
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
        let i = j - 1;
        while i > 0 && list[i] > key {
            list[i + 1] = list[i];
            i -= 1;
        }
        list[i + 1] = key;
    }
    list
}

pub fn merge_sort(mut list: Vec<usize>, first: usize, last: usize) -> Vec<usize> {
    if first < last {
        let midpoint = first.midpoint(last);

        merge_sort(list, first, midpoint);
        merge_sort(list, midpoint + 1, last);

        Merge
    }

    list
}

fn merge(mut list: Vec<usize>, first: usize, midpoint: usize, last: usize) -> Vec<usize> {
    let n_left = midpoint - first + 1;
    let n_right = last - midpoint;
}
