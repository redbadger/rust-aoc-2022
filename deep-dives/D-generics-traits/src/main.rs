use std::{collections::HashMap, str::FromStr};

enum Failing<T, E> {
    Ok(T),
    Err(E),
}

struct MyString {
    txt: String,
}

#[derive(Debug, PartialEq, Eq)]
struct MyList(Vec<u8>);

#[derive(Debug)]
struct MyListParseError(String);

impl FromStr for MyList {
    type Err = MyListParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result: Result<Vec<u8>, _> = s.split(',').map(|digit| digit.parse()).collect();

        match result {
            Ok(vec) => Ok(MyList(vec)),
            Err(e) => Err(MyListParseError(e.to_string())),
        }
    }
}

impl From<Vec<u8>> for MyList {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

fn main() {
    // let mut my_table: HashMap<String, String> = HashMap::new();
    // let mut my_table: HashMap<MyString, String> = HashMap::new();

    // my_table.insert("x".to_string(), "Hello".to_string());

    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_list() {
        let input = "1,2,3,4,7,9,16";
        let list: MyList = input.parse().unwrap();

        assert_eq!(list, MyList(vec![1, 2, 3, 4, 7, 9, 16]))
    }

    #[test]
    fn convert_from_vec() {
        let list: MyList = vec![1, 2, 3, 4, 7, 9, 16].into();

        assert_eq!(list, MyList(vec![1, 2, 3, 4, 7, 9, 16]))
    }

    #[test]
    fn double_wrap() {
        let list = vec![1, 2, 3];

        let result = list.get(2).map(|id| list.get(*id)); // we want Option<&usize>
        let result = list.get(2).and_then(|id| list.get(*id)); // we want Option<&usize>
    }
}
