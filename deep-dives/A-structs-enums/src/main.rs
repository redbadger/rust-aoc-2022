mod shape;

use std::fmt::Display;

#[derive(Debug)]
struct MyFirstStruct {
    x: u8,
    y: f64,
    z: String,
}

impl MyFirstStruct {
    fn new() -> Self {
        Self {
            x: 123,
            y: 456.789,
            z: String::from("xyz"),
        }
    }
    fn say_z(&self) {
        println!("z: {}", self.z)
    }
}

// We can create a struct from within a bare function
fn new_thing() -> MyFirstStruct {
    MyFirstStruct {
        x: 123,
        y: 456.789,
        z: String::from("xyz"),
    }
}

// Custom 'Display' trait implementation
// allows the type to be 'printed'
impl Display for MyFirstStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "x * y = {}", self.x as f64 * self.y)
    }
}

#[derive(Debug)]
enum Enth {
    First(u32),
    Second(String),
    Third { my_struct: MyFirstStruct },
}

fn main() {
    let my_struct = MyFirstStruct {
        x: 123,
        y: 5.67,
        z: String::from("it"),
    };
    println!("debug print: {:?}", my_struct);
    println!("display print: {}", my_struct);

    let nth = Enth::First(123);
    println!("{:?}", nth); // debug-print

    let nth3 = Enth::Third {
        my_struct: MyFirstStruct::new(),
    };
    let nth_literal: String = match nth3 {
        Enth::First(num) => format!("{num}"),
        Enth::Second(name) => name,
        // Enth::Third { my_struct } => format!("{my_struct}"),
        // Enth::Third { my_struct } => my_struct.z,
        Enth::Third {
            my_struct: MyFirstStruct { z, .. },
        } => z,
    };
    println!("{nth_literal}")
}
