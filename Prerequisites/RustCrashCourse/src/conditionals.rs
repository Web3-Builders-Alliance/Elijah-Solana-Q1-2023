pub fn run() {
    let age = 22;
    let check_id: bool = true;
    let knows_person_of_age = true;

    if age >= 21 && check_id || knows_person_of_age {
        println!("What would you like to drink?");
    } else if age >= 21 && !check_id   {
        println!("Bring your id first.");
    } else {
        println!("You're too young"); 
    }

    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
}