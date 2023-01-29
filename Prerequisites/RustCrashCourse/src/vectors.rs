use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    numbers[0] = 2;

    numbers.push(6);
    println!("{numbers:?}");
    numbers.pop();

    println!(
        "{:?} \nfirst element: {} \nlen: {} \nvec occupies {} bytes \na slice: {:?}", 
        numbers, numbers[0], numbers.len(), mem::size_of_val(&numbers), &numbers[1..3]
    );

    for num in &numbers {
        println!("Number: {num}");
    }

    for num in numbers.iter_mut() {
        *num *= 2;
    }

    println!("{numbers:?}");
}