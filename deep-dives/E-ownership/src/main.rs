#[derive(Clone, Debug)]
struct MyStruct {
    x: u32,
    y: String,
}

impl MyStruct {
    fn new() -> Self {
        Self {
            x: 123,
            y: "welcome".to_string(),
        }
    }

    fn say_hello(&self) {
        println!("Hello {} {}", self.x, self.y);
    }
}

fn takes_borrow(val: &MyStruct) -> &str {
    // immutable borrow
    // val.x = 10;  // error
    &val.y
}

fn takes_mut(val: &mut MyStruct) {
    // can mutate inside
    val.x = 10;
}

fn takes_owned(val: MyStruct) {
    // even if not declared `mut`, can 'cast' to mut;
    let mut val2 = val;
    takes_mut(&mut val2)
}

// fn returns_reference() -> &String {
//     let s = String::from("xyz");
//     &s  // error! Dangling reference
// }

fn bad_borrow() {
    let mut mine = MyStruct::new();
    takes_borrow(&mine); // pass by immutable reference
    takes_mut(&mut mine); // pass by mutable reference
    takes_owned(mine); // pass by move

    // takes_borrow(&mine);
    // value has been MOVED - it has GONE - and we can't access it again
    // 'affine' type system
}

fn vec_borrow() {
    let mut v = vec![1, 2, 3, 4];
    let vref = &v[0];
    // v.push(5); // error! cannot borrow mutably and immutably
    // ^^ this would be dangerous, because a vector could reallocate
    // this would cause vref to dangle, leading to a use-after-free
    println!("{vref}");
}

#[derive(Debug)]
struct OtherStruct {
    x: i32,
}

impl Drop for OtherStruct {
    fn drop(&mut self) {
        println!("Being dropped! {self:?}");
    }
}

fn create_o() {
    println!("create o");
    let o = OtherStruct { x: 123 };
    println!("leave create o");
    // o is dropped here
}

#[derive(Debug)]
struct HoldBorrow<'a> {
    borrowed_struct: &'a MyStruct,
}

fn hold_the_borrow() {
    let my_struct = MyStruct::new();
    let hold = HoldBorrow {
        borrowed_struct: &my_struct,
    };
    my_struct.say_hello();
    println!("{hold:?}");
}

fn main() {
    bad_borrow();
    create_o()
}

struct Node<T> {
    value: T,
    child: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn push(&mut self, value: T) {
        if let Some(next) = &mut self.child {
            next.push(value)
        } else {
            self.child = Some(Box::new(Node { value, child: None }));
        }
    }
}

struct SinglyLinkedList<T> {
    root: Option<Node<T>>,
}

impl<T> SinglyLinkedList<T> {
    fn new() -> Self {
        Self { root: None }
    }
    fn push(&mut self, value: T) {
        if let Some(root) = &mut self.root {
            root.push(value)
        } else {
            self.root = Some(Node { value, child: None });
        }
    }
}

fn make_linked_list() {
    let mut ll = SinglyLinkedList::new();
    for x in [1, 2, 3] {
        ll.push(x)
    }
}
