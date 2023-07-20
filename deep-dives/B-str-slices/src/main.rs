#[derive(Clone, Debug)]
struct MyStruct {
    x: u32,
    y: String,
}

fn takes_borrow(val: &MyStruct) -> &String {
    &val.y
}
fn takes_mut(val: &mut MyStruct) {
    val.x = 10;
}
fn takes_owned(val: MyStruct) {
    let mut val2 = val;
    takes_mut(&mut val2)
}

fn main() {
    let mut s = MyStruct {
        x: 10,
        y: "Hello".into(),
    };

    // takes_owned(s);
    let sy = takes_borrow(&s);
    // takes_mut(&mut s);

    println!("{sy:?}");
}

fn split(s: &str, delimiter: char) -> Vec<&str> {
    let mut the_splits = Vec::new();
    let mut prev_ix = 0;
    for (ix, c) in s.chars().enumerate() {
        if c == delimiter {
            let range = prev_ix..ix;

            let slice = &s[range];
            // equivalent
            // let slice = s.get(range).unwrap();

            the_splits.push(slice);
            prev_ix = ix + 1;
        }
    }

    let range = prev_ix..s.len();
    let slice = &s[range];
    the_splits.push(slice);
    the_splits
}

#[test]
fn test_split() {
    let test = "a cat sat on a mat";
    let result = split(test, ' ');
    let expect = vec!["a", "cat", "sat", "on", "a", "mat"];
    assert_eq!(result, expect);

    let test = " a mat ";
    let result = split(test, ' ');
    let expect = vec!["", "a", "mat", ""];
    assert_eq!(result, expect);

    let test = "";
    let result = split(test, ' ');
    let expect = vec![""];
    assert_eq!(result, expect);
}
