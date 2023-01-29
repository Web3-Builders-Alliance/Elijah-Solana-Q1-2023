pub fn run() {
    let person: (String, &str, u8) = ("Bred".to_owned(), "Mass", 12);

    println!("{} is from {} and is {} years old.", person.0, person.1, person.2);
}