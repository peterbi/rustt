struct Color {
    red: u8,
    green: u8,
    blue: u8
}

struct Color1(u8, u8, u8);

struct Person {
    fname: String,
    lname: String
}

impl Person {
    // Construct
    fn new(first: &str, last: &str) -> Person {
        Person{
            fname: first.to_string(),
            lname: last.to_string()
        }
    }

    // full nae
    fn full_name(&self) -> String {
        format!("{} {}", self.fname, self.lname)
    }

    // set last name
    fn set_last_name(&mut self, last: &str) {
        self.lname = last.to_string();
    }

    // to tuple
    fn to_tuple(self) -> (String, String) {
        (self.fname, self.lname)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue:0
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c = Color1(255, 0, 0);
    c.0 = 2;
    println!("Color {} {} {}", c.0, c.1, c.2);

    let mut p=Person::new("john", "done");
    p.set_last_name("Doe");
    println!("Person {} {}", p.fname, p.lname);
    println!("Person {}", p.full_name());
    println!("Person {:?}", p.to_tuple())

}