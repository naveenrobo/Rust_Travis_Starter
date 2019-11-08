// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct
struct Color1(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut color = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    color.red = 200;

    println!("Color : {} {} {}", color.red, color.green, color.blue);

    let mut c = Color1(255, 255, 255);
    c.0 = 200;
    println!("Color is {} {} {}", c.0, c.1, c.2);

    let mut person = Person::new("Naveen", "Sakthivel");
    person.set_last_name(" ");
    println!("Person {}", person.full_name());
    println!("Person {:?}", person.to_tuple());
}
