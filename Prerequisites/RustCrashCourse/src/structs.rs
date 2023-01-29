// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// struct Color(u8, u8, u8);

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    fn new(first_name: String, last_name: String) -> Self {
        Self {
            first_name,
            last_name
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last_name: String) {
        self.last_name = last_name;
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // let mut color = Color {
    //     red: 255,
    //     green: 255,
    //     blue: 255,
    // }; 

    // println!("Color: {}, {}, {}.", color.red, color.green, color.blue);
    
    // color.red = 200;
    // println!("Color: {}, {}, {}.", color.red, color.green, color.blue);
    // let mut color = Color(255, 255, 255);
    // color.0 = 200;
    // println!("Color: {}, {}, {}.", color.0, color.1, color.2);

    let mut elijah = Person::new(String::from("Elijah"), "Test".to_owned());

    elijah.last_name = String::from("Test2");

    println!("{:?}", elijah);
    println!("{}", elijah.full_name());

    elijah.set_last_name("Brown".to_owned());
    println!("The last name has been changed: {}", elijah.full_name());

    println!("{:?}", elijah.to_tuple());

}
