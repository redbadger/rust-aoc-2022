fn main() {
    let mut v = vec![1, 2, 3, 4];
    v.push(5);

    add_the_vec_value(&v);

    // coerce vec to slice
    add_the_value(&v);
    // or convert explicitly
    add_the_value(v.as_slice());

    // taking a subslice
    let range = 1..3;
    let my_slice = &v[range];
    add_the_value(my_slice);

    // subslice of subslice
    add_the_value(&my_slice[2..3]);

    // we can make a slice from a string
    let s = String::from("123");
    let byteslice = s.as_bytes();
    add_the_value(byteslice);

    // slice literal
    add_the_value(&[1, 2, 3]);

    // empty slice literal
    add_the_value(&[]);
}

// This would be unusual - an immutable vec
// is strictly less useful than a slice
fn add_the_vec_value(v: &Vec<u8>) -> u8 {
    v.iter().sum()
}

fn add_the_value(v: &[u8]) -> u8 {
    //////////////// ^^^^ slice syntax
    v.iter().sum()
}

// Again, this would be unusual - an immutable String
// is strictly less useful than a str
fn print_my_string(s: &String) {
    println!("{s}")
}

fn print_my_str(s: &str) {
    println!("{s}")
}
