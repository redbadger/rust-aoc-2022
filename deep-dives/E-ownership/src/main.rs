fn main() {
    bad_borrow();
    create_o()
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
