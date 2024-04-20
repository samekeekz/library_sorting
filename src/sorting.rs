use std::cmp::Ordering;


pub fn quick_sort<T, F>(arr: &mut [T], compare: &F)
where
    T: Clone,
    F: Fn(&T, &T) -> Ordering,
{
    if arr.len() <= 1 {
        return;
    }
    let pivot = arr[arr.len() / 2].clone();
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        while compare(&arr[left], &pivot) == Ordering::Less {
            left += 1;
        }
        while compare(&arr[right], &pivot) == Ordering::Greater {
            right -= 1;
        }
        if left <= right {
            arr.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    if right > 0 {
        quick_sort(&mut arr[0..=right], compare);
    }
    if left < arr.len() {
        quick_sort(&mut arr[left..], compare);
    }
}


pub fn selection_sort<T, F>(arr: &mut [T], compare: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let len = arr.len();
    for i in 0..len {
        let mut min_index = i;
        for j in (i + 1)..len {
            if compare(&arr[j], &arr[min_index]) == Ordering::Less {
                min_index = j;
            }
        }
        if min_index != i {
            arr.swap(i, min_index);
        }
    }
}


pub fn insertion_sort<T, F>(arr: &mut [T], compare: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && compare(&arr[j - 1], &arr[j]) == Ordering::Greater {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn merge_sort<T, F>(arr: &mut [T], compare: &F)
where
    T: Clone,
    F: Fn(&T, &T) -> Ordering,
{
    let len = arr.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge_sort(&mut left, compare);
    merge_sort(&mut right, compare);

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if compare(&left[i], &right[j]) == Ordering::Less {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}
