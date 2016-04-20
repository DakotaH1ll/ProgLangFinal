fn bubble_sort<T: Ord>(values: &mut[T]) {
    let mut n = values.len();
    let mut swapped = true;

    while swapped {
        swapped = false;

        for i in 1..n {
            if values[i - 1] > values[i] {
                values.swap(i - 1, i);
                swapped = true;
            }
        }

        n = n - 1;
    }
}

fn main() {
let less_start_time = time::precise_time_ns();

// Sort numbers
println!("");
let mut numbers = [80, 32, 19, 56, -15, 1, -1, 0, 782, 1];
println!("US: {:?}", numbers);

bubble_sort(&mut numbers);
println!("S: {:?}", numbers);

// Sort strings
println!("");
let mut strings = ["Hulk", "Captain America", "Black Widow", "Iron Man", "Thor", "Hawkeye"];
println!("US: {:?}", strings);

bubble_sort(&mut strings);
println!("S: {:?}", strings);

//sort chars
println!("");
let mut chars = ["b", "A", "h", "Z", "Q", "q"];
println!("US: {:?}", chars);

bubble_sort(&mut chars);
println!("S: {:?}", chars);

let less_end_time = time::precise_time_ns();


//Testing Worst Case Scenario (One Unsorted Object at End Of List)
let worst_start_time = time::precise_time_ns();
println!("");

let mut worst_numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
println!("US: {:?}", worst_numbers);

bubble_sort(&mut worst_numbers);
println!("S: {:?}", worst_numbers);

let worst_end_time = time::precise_time_ns();

println!("");

println!("Time Taken (Three Sorts): {} Nanoseconds", less_end_time-less_start_time);
println!("Time Taken (Worst Case Scenario): {} Nanoseconds", worst_end_time-worst_start_time);


}
