// use std::ops::Add;

use std::ops::Deref;

fn main() {
    println!("hello");
    println!("hello {} {}", 1, 2);

    let s = MyStruct::new();
    let json = serde_json::to_string(&s).unwrap();
    println!("{json}");

    println!("{}", hello());
    println!("{}", goodbye());
    println!("{}", welcome());

    let user_id = UserId("id1".to_string());
    let user_id_str: &str = &*user_id;
    let user_id_str: &str = user_id.deref(); // <- equivalent

    println!("user id: {}", user_id.deref());
    println!("content id: {}", ContentId("hi".into()).deref());
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "UPPERCASE")]
struct MyStruct {
    #[serde(default)]
    x: i32,
    y: String,
}

impl MyStruct {
    fn new() -> Self {
        Self {
            x: 123,
            y: "hi".into(),
        }
    }
}

macro_rules! make_func {
    ($name: ident, $val: expr) => {
        fn $name() -> &'static str {
            $val
        }
    };
    ($name: ident) => {
        make_func!($name, "welcome to macro-world");
    };
}

make_func!(hello, "hello world");
make_func!(goodbye, "goodbye cruel world");
make_func!(welcome);

// newtype pattern

macro_rules! newtype {
    ($name: ident) => {
        struct $name(String);

        impl Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

newtype!(UserId);
newtype!(CommentId);
newtype!(ContentId);

// Example of 'overloading' via the plus operator

// struct A;
// struct B;

// impl Add for A {
//     type Output = A;
//     fn add(self, rhs: Self) -> Self::Output {
//         todo!()
//     }
// }

// impl Add<B> for A {
//     type Output = A;
//     fn add(self, rhs: B) -> Self::Output {
//         todo!()
//     }
// }

// fn do_the_add() {
//     A + A;
//     A + B;   // overloading!
// }
