
// normal struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// tuple struct
struct Color2(u8, u8, u8);

// basic type with ctor and method...
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // construct Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // note: &mut self reference because we modify the object...
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(&self) -> (String, String) {
        (self.first_name.to_string(), self.last_name.to_string())
    }
}

pub fn run() {
    let c = Color { red: 255, green: 0, blue: 0 };
    println!("{} {} {}", c.red, c.green, c.blue);

    let c2 = Color2(0, 255, 0);
    println!("{} {} {}", c2.0, c2.1, c2.2);

    let mut p = Person::new("Tony", "Di Croce");
    println!("{} {}", p.first_name, p.last_name);
    println!("{}", p.full_name());
    p.set_last_name("weasel");
    println!("{}", p.full_name());

    let fields = p.to_tuple();
    println!("{:?}", fields);
}
