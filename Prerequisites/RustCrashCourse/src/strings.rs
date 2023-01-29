pub fn run() {
    let hello = "str";
    let mut hello = "hello ".to_owned(); // shadowing

    //println!("The word {} has {} chars", hello, hello.len());

    hello.push('s');
    hello.push_str("tring");

    //println!("The sentence  {} has {} chars", hello, hello.len());

    // Capacity in bytes
    println!(
        "Capacity: {}, is_empty: {}, contains 'tri': {}, replaced: {}", 
        hello.capacity(), hello.is_empty(), hello.contains("tri"), hello
        .replace("string", "world!")
    );

    for word in hello.split(" ") {
        println!("{}", word);
    }

    //String with a particular capacity
    let mut s = String::with_capacity(12); 
    s.push('a');
    s.push('b');
    assert_eq!(2, s.len());
    assert_eq!(12, s.capacity());

    println!("{}", s);
}