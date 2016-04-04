fn main() {
    println!("Dakota Hill's Testing Suite");
    println!("Current Test: Vectors");
    println!("---------------------------");

    let mut vec = Vec::new();
    vec.push("Bruce Wayne");
    vec.push("Clark Kent");
    vec.push("Diana Prince");
    vec.push("Hal Jordan");
    vec.push("Barry Allen");
    vec.push("Arthur Curry");
    vec.push("Victor Stone");

    for x in &vec {
    println!("{}", x);
    }
}
