mod sorting;
use sorting::{quick_sort, selection_sort, insertion_sort, merge_sort};
use std::cmp::Ordering;

fn main() {
    // Example usage of quick sort
    let mut nums = vec![4, 2, 5, 1, 3];
    quick_sort(&mut nums, &|a, b| a.cmp(b));
    println!("Quick Sort: {:?}", nums);

    // Example usage of selection sort
    let mut nums = vec![4, 2, 5, 1, 3];
    selection_sort(&mut nums, &|a, b| a.cmp(b));
    println!("Selection Sort: {:?}", nums);

    // Example usage of insertion sort
    let mut nums = vec![4, 2, 5, 1, 3];
    insertion_sort(&mut nums, &|a, b| a.cmp(b));
    println!("Insertion Sort: {:?}", nums);

    // Example usage of merge sort
    let mut nums = vec![9,8,6,4,2,62,6];
    merge_sort(&mut nums, &|a, b| a.cmp(b));
    println!("Merge Sort: {:?}", nums);
}
