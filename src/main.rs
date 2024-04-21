use sorting::{quick_sort, selection_sort, insertion_sort, merge_sort};

mod sorting;

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct Pokemon {
        name: String,
        level: u32,
    }


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

    let mut nums = vec![9, 8, 6, 4, 2, 62, 6];
    merge_sort(&mut nums, &|a, b| a.cmp(b));
    println!("Merge Sort: {:?}", nums);

    let mut strings = vec!["banana", "apple", "cherry", "date", "elderberry"];
    quick_sort(&mut strings, &|a, b| a.cmp(b));
    println!("Quick Sort: {:?}", strings);

    let mut pokemons = vec![
        Pokemon {
            name: "Pikachu".to_string(),
            level: 25,
        },
        Pokemon {
            name: "Charizard".to_string(),
            level: 50,
        },
        Pokemon {
            name: "Bulbasaur".to_string(),
            level: 10,
        },
    ];

    quick_sort(&mut pokemons, &|a, b| a.level.cmp(&b.level));
    println!("Quick Sort (by level): {:?}", pokemons);

     quick_sort(&mut pokemons, &|a, b| a.name.cmp(&b.name));
    println!("Quick Sort (by level): {:?}", pokemons);


    let mut reversed_strings = vec!["banana", "apple", "cherry", "date", "elderberry"];
    quick_sort(&mut reversed_strings, &|a, b| b.cmp(a));
    println!("Quick Sort (reversed): {:?}", reversed_strings);
}
