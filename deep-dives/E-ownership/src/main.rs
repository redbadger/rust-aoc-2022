use std::{fs::File, io::Read, rc::Rc};

fn main() {
    // vec_rc();
    // bad_borrow();
    create_and_drop_other();
}

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
    // val.x = 10; // error
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
    let vref = &v[3];
    v.push(5); // error! cannot borrow mutably and immutably
               // ^^ this would be dangerous, because a vector could reallocate
               // this would cause vref to dangle, leading to a use-after-free
               // println!("{vref}");
}

fn vec_copy() {
    let mut v = vec![MyStruct::new()];
    let vref = v[0].clone();
    v.push(MyStruct::new()); // error! cannot borrow mutably and immutably
                             // ^^ this would be dangerous, because a vector could reallocate
                             // this would cause vref to dangle, leading to a use-after-free
    println!("{vref:?}");
}

fn vec_rc() {
    let my_struct = Rc::new(MyStruct::new());
    println!("{}", Rc::strong_count(&my_struct));

    let mut v = vec![my_struct];

    let vref = v[0].clone();
    println!("{}", Rc::strong_count(&vref));

    let vref2 = v[0].clone();
    println!("{}", Rc::strong_count(&vref));

    v.push(Rc::new(MyStruct::new())); // error! cannot borrow mutably and immutably
                                      // ^^ this would be dangerous, because a vector could reallocate
                                      // this would cause vref to dangle, leading to a use-after-free
                                      // println!("{vref:?}",);
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

fn create_and_drop_other() {
    println!("create o");
    let o = OtherStruct { x: 123 };
    // take_other_struct_by_move(o);
    take_other_struct_by_ref(&o);
    println!("leave create o");
    // o is dropped here
}

fn take_other_struct_by_move(val: OtherStruct) {
    println!("I got {:?}", val);
}

fn take_other_struct_by_ref(val: &OtherStruct) {
    println!("I got {:?}", val);
}

fn take_one_borrows(s1: &str) -> &str {
    s1
}

fn take_two_borrows_no_return(s1: &str, s2: &str) {
    println!("{s1} {s2}");
}

fn take_two_borrows_and_returns_first<'a, 'b>(s1: &'a str, s2: &'b str) -> &'a str {
    println!("{s1} {s2}");
    s1
}

fn take_two_borrows_and_returns_second<'a, 'b: 'a>(s1: &'a str, s2: &'b str) -> &'a str {
    println!("{s1} {s2}");
    s2
}

fn take_two_borrows_returns_int(s1: &str, s2: &str) -> u32 {
    println!("{s1} {s2}");
    123
}

fn takes_static(s: &'static str) {
    println!("{s}");
}

fn calls_takes_static() {
    let my_str: &'static str = "Hello world";
    takes_static(my_str);

    // let new_thing = String::from("Goodbye world");
    // let new_thing_slice: &str = &new_thing;
    // takes_static(new_thing_slice); // <- won't work - wrong lifetime

    println!("{my_str}")
}

// fn take_two_borrows_and_returns_either<'a>(s1: &'a str, s2: &'a str) -> &'a str {
//     println!("{s1} {s2}");
//     if rand::random() > 0.5 {
//         s2
//     } else {
//         s1
//     }
// }

#[derive(Debug)]
struct HoldBorrow<'a> {
    borrowed_struct: &'a MyStruct,
}

fn hold_the_borrow() {
    let mut my_struct = MyStruct::new();
    let hold = HoldBorrow {
        borrowed_struct: &my_struct,
    };

    my_struct.say_hello();

    // my_struct.x = 345;

    println!("{hold:?}");
}

fn open_a_file() {
    let mut file = std::fs::File::open("fake-file.txt").unwrap();
    let output = read_the_file(file);
    // let output = read_the_file(file); // <-  Error! file has been moved (and dropped)
}

fn read_the_file(mut file: File) -> String {
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf
}

struct SinglyLinkedList<T> {
    root: Node<T>,
}

impl<T> SinglyLinkedList<T> {
    fn new() -> Self {
        Self { root: Node::Empty }
    }
    fn push(&mut self, value: T) {
        self.root.push(value)
    }
    fn pop(&mut self) -> Option<T> {
        self.root.pop()
    }
}

enum Node<T> {
    Empty,
    Link { value: T, next: Box<Self> },
}

impl<T> Node<T> {
    fn push(&mut self, value: T) {
        match self {
            Node::Empty => {
                *self = Node::Link {
                    value,
                    next: Box::new(Node::Empty),
                }
            }
            Node::Link { next, .. } => next.push(value),
        }
    }
    fn pop(&mut self) -> Option<T> {
        match self {
            Node::Empty => None,
            Node::Link { next, value } => match **next {
                Node::Link { .. } => next.pop(),
                Node::Empty => {
                    let mut extract = Node::Empty;
                    std::mem::swap(self, &mut extract);
                    match extract {
                        Node::Link { value, .. } => Some(value),
                        Node::Empty => unreachable!(),
                    }
                }
            },
        }
    }
}

fn demo_linked_list() {
    let mut ll = SinglyLinkedList::new();
    for x in [1, 2, 3] {
        ll.push(x)
    }
}

struct DNode<T> {
    parent: Box<DNode<T>>,
    item: DNodeItem<T>,
}

impl<T> DNode<T> {
    fn push(&mut self, val: T) {
        match &mut self.item {
            DNodeItem::Empty => {}
            DNodeItem::Link { child, .. } => child.push(val),
        }
    }
}

enum DNodeItem<T> {
    Empty,
    Link { value: T, child: Box<DNode<T>> },
}

struct DoublyLinkedList<T> {
    front: DNodeItem<T>,
    back: DNodeItem<T>,
}

impl<T> DoublyLinkedList<T> {
    fn new() -> Self {
        Self {
            front: DNodeItem::Empty,
            back: DNodeItem::Empty,
        }
    }
}
