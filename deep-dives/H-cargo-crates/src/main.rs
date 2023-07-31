fn main() {
    #[cfg(feature = "add-numbers")]
    {
        println!("I am adding numbers!");
        assert_eq!(add_numbers(1, 2), 3);
    }
    #[cfg(feature = "multiply-numbers")]
    {
        println!("I am multiplying numbers!");
        assert_eq!(multiply_numbers(2, 3), 6);
    }

    let message = hello::message();
    println!("{message}");
}

#[cfg(feature = "add-numbers")]
fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(feature = "multiply-numbers")]
fn multiply_numbers(x: i32, y: i32) -> i32 {
    x * y
}

mod hello {
    include!(concat!(env!("OUT_DIR"), "/hello.rs"));
}
