pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    numbers[2] = 20;

    numbers.push(5);

    println!("{:?}", numbers);

    println!("{}", numbers[0]);

    //Arrays are stack allocated
    println!("Vec occupies {} bytes", std::mem::size_of_val(&numbers));

        // get slice
        let slice: &[i32] = &numbers[1..2];
        println!("Slice: {:?}", slice);

        // loop
        for x in numbers.iter() {
            println!("Number {}", x);
        }

        // loop mut
        for x in numbers.iter_mut() {
            *x *= 2;
        }

        println!("Numbers Vec: {:?}", numbers);
}