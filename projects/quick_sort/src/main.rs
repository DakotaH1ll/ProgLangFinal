extern crate time;
// Type alias for function that returns true if arguments should be swapped
type OrderFunc<T> = Fn(&T, &T) -> bool;

fn main() {
    let less_start_time = time::precise_time_ns();

    // Sort numbers
    println!("");
    let mut numbers = [80, 32, 19, 56, -15, 1, -1, 0, 782, 1];
    println!("US: {:?}", numbers);

    quick_sort(&mut numbers, &is_less);
    println!("S: {:?}", numbers);

    // Sort strings
    println!("");
    let mut strings = ["Hulk", "Captain America", "Black Widow", "Iron Man", "Thor", "Hawkeye"];
    println!("US: {:?}", strings);

    quick_sort(&mut strings, &is_less);
    println!("S: {:?}", strings);

    //sort chars
    println!("");
    let mut chars = ["b", "A", "h", "Z", "Q", "q"];
    println!("US: {:?}", chars);

    quick_sort(&mut chars, &is_less);
    println!("S: {:?}", chars);

    let less_end_time = time::precise_time_ns();


//Testing Worst Case Scenario (One Unsorted Object at End Of List)
    let worst_start_time = time::precise_time_ns();
    println!("");

    let mut worst_numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    println!("US: {:?}", worst_numbers);

    quick_sort(&mut worst_numbers, &is_less);
    println!("S: {:?}", worst_numbers);

    let worst_end_time = time::precise_time_ns();

    println!("");

    println!("Time Taken (Three Sorts): {} Nanoseconds", less_end_time-less_start_time);
    println!("Time Taken (Worst Case Scenario): {} Nanoseconds", worst_end_time-worst_start_time);

}


// Example OrderFunc which is used to order items from least to greatest
#[inline(always)]
fn is_less<T: Ord>(x: &T, y: &T) -> bool {
    x < y
}

fn is_more<T: Ord>(x: &T, y: &T) -> bool {
    x > y
}

fn quick_sort<T>(group: &mut [T], function: &OrderFunc<T>) {

    let len = group.len();
    if len < 2 {
        return;
    }

    let pivot_index = partition(group, function);

    // Sort the left side
    quick_sort(&mut group[0..pivot_index], function);

    // Sort the right side
    quick_sort(&mut group[pivot_index + 1..len], function);
}

fn partition<T>(group: &mut [T], function: &OrderFunc<T>) -> usize {
    let len = group.len();
    let pivot_index = len / 2;

    group.swap(pivot_index, len - 1);

    let mut store_index = 0;
    for i in 0..len - 1 {
        if function(&group[i], &group[len - 1]) {
            group.swap(i, store_index);
            store_index += 1;
        }
    }

    group.swap(store_index, len - 1);
    store_index
}
