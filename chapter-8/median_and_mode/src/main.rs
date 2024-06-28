/*
Decided to do these because of the note at the end of the "8.3 Exercises"
This is the first of those

BTW, the median works fine, but the mode isn't the best and doesn't work if theres more than one mode...

*/

fn main() {
    let mut list = vec![12,12,9,8,1,13,13];

    list.sort();

    // FINDING THE MEDIAN

    let length_of_list = list.len();
    let median:f64;

    if length_of_list % 2 != 0 {
        let middle_for_odd_index = (((&length_of_list + 1) / 2) - 1) as usize;
        median = list[middle_for_odd_index] as f64;
    } 
    else {
        let top_bound_even = &length_of_list/2;
        let bottom_bound_even = &top_bound_even - 1;
        let median_even = (list[top_bound_even] + list[bottom_bound_even]) as f64 / 2.0;
        median = median_even;
    }

    println!("The median is: {median}");

    // FINDING THE MODE
    use std::collections::HashMap;
    let mut map = HashMap::new();

    for number in &list {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }

    let mut max_count = 0;
    let mut mode:i32 = 0;

    // Find the mode (number with maximum occurrences)
    for (key, &value) in &map {
        println!("Key: {key}, Value: {value}");
        if value > max_count {
            max_count = value;
            mode = **key;
            
        }
    }

    println!("The mode is: {}", mode);
}