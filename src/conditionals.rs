pub fn run() {
    let age = 18;

    let check_id: bool = false;

    if age >= 21 && check_id {
        println!("Bartender says, what would you like to drink");
    } else if age < 21 && check_id {
        println!("Bartender says you'll have to leave");
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    let mut count = 0;
    loop {
        count += 1;
         println!("{}", count);
         if count == 20 {
             break;
         }
    }

    while count <= 100 {
        if count % 15 == 0 {
            println!("fb");
        } else if count % 3 == 0 {
            println!("f");
        } else if count % 5 == 0 {
            println!("b");
        } else {
            println!("{}", count);
        }

        count += 1;
    }

    for x in 0..100 {
        
        if x % 15 == 0 {
            println!("fb");
        } else if x % 3 == 0 {
            println!("f");
        } else if x % 5 == 0 {
            println!("b");
        } else {
            println!("{}", x);
        }

    }
}