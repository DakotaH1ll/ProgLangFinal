fn main() {
    println!("Dakota Hill's Testing Suite");
    println!("Current Test: Vectors");
    println!("---------------------------");

    let mut vec = Vec::new();
    vec.push("Thor");
    vec.push("Black Widow");
    vec.push("Iron Man");
    vec.push("Hawkeye");
    vec.push("Captain America");
    vec.push("Hulk");

    let mut vec2 = Vec::new();
    vec2.push("Bruce Banner");
    vec2.push("Clint Barton");
    vec2.push("Steve Rogers");
    vec2.push("Thor Odinson");
    vec2.push("Tony Stark");
    vec2.push("Natasha Romanoff");

    println!("{} is {}", vec2[0], vec[0]);
    println!("{} is {}", vec2[1], vec[4]);
    println!("{} is {}", vec2[2], vec[2]);
    println!("{} is {}", vec2[3], vec[5]);
    println!("{} is {}", vec2[4], vec[1]);
    println!("{} is {}", vec2[5], vec[3]);
    println!("");

    println!("Wait a minute...");

    vec.sort();
    vec2.sort();
    println!("");

    println!("{} is {}", vec2[0], vec[3]);
    println!("{} is {}", vec2[1], vec[2]);
    println!("{} is {}", vec2[2], vec[0]);
    println!("{} is {}", vec2[3], vec[1]);
    println!("{} is {}", vec2[4], vec[5]);
    println!("{} is {}", vec2[5], vec[4]);

}
