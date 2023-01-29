pub fn run() {
    let name = "Brad";
    let mut age = 37;
    age = 38;

    println!("My name is {} and I'm {} years old.", name, age);

    const ID: u32 = 001;
    println!("ID: {}", ID);

    let (my_name, my_age) = ("Brad", 32);
    println!("{} is {}", my_name, my_age);
}