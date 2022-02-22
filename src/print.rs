use std::mem;

pub fn run() {
    println!("Hello from the print rs file");

    // arg
    println!("{}", 1);

    // pos arg
    println!("{0} is from {1} and {0} {2}", "brad", "mass", "codes");

    // place holder
    println!{"bin: {:b} hex: {:x}", 10, 10};

    // debug
    println!("{:?}", (12, true));


}