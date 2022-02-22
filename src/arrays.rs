pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    numbers[2] = 20;

    println!("{:?}", numbers);

    println!("{}", numbers[0]);

    //Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

        // get slice
        let slice: &[i32] = &numbers[1..2];
        println!("Slice: {:?}", slice);
}