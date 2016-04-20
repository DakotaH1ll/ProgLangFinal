fn insertion_sort<T: std::cmp::Ord>(items: &mut [T]) {
    for i in 1..items.len() {
        let mut j = i;
        while j > 0 && items[j] < items[j-1] {
            items.swap(j, j-1);
            j = j-1;
        }
    }
}

fn main() {

    // Sort numbers
    println!("");
    let mut numbers = ["Captain America", "Thor", "Iron Man", "Black Widow", "Hulk", "Hawkeye"];
    println!("US: {:?}", numbers);

    insertion_sort(&mut numbers);
    println!("S: {:?}", numbers);
}
