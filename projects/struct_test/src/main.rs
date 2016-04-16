struct Person {
    first_name: &'static str,
    last_name: &'static str,
}

struct Team {
  leader: Person,
  member: Person,
}

fn main() {
    println!("Dakota Hill's Testing Suite");
    println!("Current Test: Structs");
    println!("---------------------------");

    let iron_man = Person { first_name: "Tony", last_name: "Stark" };
    let war_machine = Person { first_name: "James", last_name: "Rhodes" };

    let captain_america = Person { first_name: "Steve", last_name: "Rogers" };
    let winter_soldier = Person { first_name: "Bucky", last_name: "Barnes" };

    let team_tony = Team { leader: iron_man, member: war_machine};
    let team_cap = Team { leader: captain_america, member: winter_soldier};

    println!("Team Iron Man is lead by {} {}", team_tony.leader.first_name, team_tony.leader.last_name);
    println!("He leads {} {}", team_tony.member.first_name, team_tony.member.last_name);
    println!("Team Captain America is lead by {} {}", team_cap.leader.first_name, team_cap.leader.last_name);
    println!("He leads {} {}", team_cap.member.first_name, team_cap.member.last_name);


}
