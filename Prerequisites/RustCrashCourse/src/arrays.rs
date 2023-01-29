use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    numbers[0] = 2;

    println!(
        "{:?} \nfirst element: {} \nlen: {} \narray occupies {} bytes \na slice: {:?}", 
        numbers, numbers[0], numbers.len(), mem::size_of_val(&numbers), &numbers[1..3]
    );
}