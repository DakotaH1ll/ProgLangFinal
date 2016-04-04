fn main() {
    println!("Dakota Hill's Testing Suite");
    println!("Current Test: Vectors");
    println!("---------------------------");

    let mut vec = Vec::new();
    vec.push("Aquaman");
    vec.push("Batman");
    vec.push("Cyborg");
    vec.push("Flash");
    vec.push("Green Lantern");
    vec.push("Superman");
    vec.push("Wonder Woman");

    let mut vec2 = Vec::new();
    vec2.push("Arthur Curry");
    vec2.push("Barry Allen");
    vec2.push("Bruce Wayne");
    vec2.push("Clark Kent");
    vec2.push("Diana Prince");
    vec2.push("Hal Jordan");
    vec2.push("Victor Stone");



    println!("{} is {}", vec2[0], vec[0]);
    println!("{} is {}", vec2[1], vec[3]);
    println!("{} is {}", vec2[2], vec[1]);
    println!("{} is {}", vec2[3], vec[5]);
    println!("{} is {}", vec2[4], vec[6]);
    println!("{} is {}", vec2[5], vec[4]);
    println!("{} is {}", vec2[6], vec[2]);

}
