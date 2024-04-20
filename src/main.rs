mod sorting;
use sorting::{quick_sort, selection_sort, insertion_sort, merge_sort};
use std::cmp::Ordering;

fn main() {
    let mut nums = vec![4, 2, 5, 1, 3];
    quick_sort(&mut nums, &|a, b| a.cmp(b));
    println!("Quick Sort: {:?}", nums);

    let mut nums = vec![4, 2, 5, 1, 3];
    selection_sort(&mut nums, &|a, b| a.cmp(b));
    println!("Selection Sort: {:?}", nums);

    let mut nums = vec![4, 2, 5, 1, 3];
    insertion_sort(&mut nums, &|a, b| a.cmp(b));
    println!("Insertion Sort: {:?}", nums);

    let mut nums = vec![9,8,6,4,2,62,6];
    merge_sort(&mut nums, &|a, b| a.cmp(b));
    println!("Merge Sort: {:?}", nums);

    let mut strings = vec!["banana", "apple", "cherry", "date", "elderberry"];
    quick_sort(&mut strings, &|a, b| a.cmp(b));
    println!("Quick Sort: {:?}", strings);

    let mut nums = vec![4, 2, 5, 1, 3];
    quick_sort(&mut nums, &|a, b| b.cmp(a));
    println!("Quick Sort (Reverse): {:?}", nums);

    let mut nums = vec![4, 2, 5, 1, 3];
    selection_sort(&mut nums, &|a, b| b.cmp(a));
    println!("Selection Sort (Reverse): {:?}", nums);

    let mut nums = vec![4, 2, 5, 1, 3];
    insertion_sort(&mut nums, &|a, b| b.cmp(a));
    println!("Insertion Sort (Reverse): {:?}", nums);

    let mut nums = vec![9,8,6,4,2,62,6];
    merge_sort(&mut nums, &|a, b| b.cmp(a));
    println!("Merge Sort (Reverse): {:?}", nums);

    let mut strings = vec!["banana", "apple", "cherry", "date", "elderberry"];
    quick_sort(&mut strings, &|a, b| b.cmp(a));
    println!("Quick Sort (Reverse): {:?}", strings);
}
