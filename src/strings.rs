pub fn run() {
    let mut hello = String::from("Hello");

    println!("Length: {}", hello.len());

    hello.push('W');

    hello.push_str("orld!");

    println!("cap {}", hello.capacity());

    println!("{}", hello);

    println!("{}", hello.replace("World", "There"));

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assert
    assert_eq!(2, s.len());

    println!("{}", s);
}