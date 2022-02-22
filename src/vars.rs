pub fn run() {
    let name = "Brad";
    let mut age = 27;

    age = 38;

    println!("my name is {} and im {}", name, age);

    //Define const
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // multi vars
    let (my_name, age) = ("ab", 37);
    println!("{}, {}", my_name, age);
}