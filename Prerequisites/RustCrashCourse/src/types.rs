pub fn run() {
    let x = 1;
    let y = 2.5;
    let z: u8 = 255;

    println!("Max u32: {}", u32::MAX);
    println!("Max i8: {}", i8::MAX);

    let is_active = true;
    let is_greater = 10 > 5;

    let a1 = 'a'; // unicode
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}