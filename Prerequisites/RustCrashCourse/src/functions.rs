pub fn run() {
    greeting("Hi", "Bred");

    let sum = add(5, 5);
    println!("Sum: {}", sum);

    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("5 + 2 + {n3} = {}", add_nums(5, 2));
}

fn greeting(greet: &str, name: &str) {
    println!("{greet}, {name}! Nice to meet you!");
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
