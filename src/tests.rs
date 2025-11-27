
use super::{selection_sort, insertion_sort, merge_sort, bogosort};

#[test]
fn test_selection_sort() {
    let list = vec![64, 25, 12, 22, 11];
    let sorted_list = selection_sort(list);
    assert_eq!(sorted_list, vec![11, 12, 22, 25, 64]);
}   
#[test]
fn test_insertion_sort() {
    let list = vec![64, 25, 12, 22, 11];
    let sorted_list = insertion_sort(list);
    assert_eq!(sorted_list, vec![11, 12, 22, 25, 64]);
}
#[test]
fn test_merge_sort() {
    let list = vec![64, 25, 12, 22, 11];
    let sorted_list = merge_sort(list, 0, 4);
    assert_eq!(sorted_list, vec![11, 12, 22, 25, 64]);
}
#[test]
fn test_bogosort() {
    let list = vec![64, 25, 12, 22, 11];
    let sorted_list = bogosort(list);
    assert_eq!(sorted_list, vec![11, 12, 22, 25, 64]);
}