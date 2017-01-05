use List::*;

enum List {
    Cons(u32, Box<List>),
    Nil
}

impl List {
    fn new() -> List {
        Nil
    }
    
    fn prepend(self, value: u32) -> List {
        Cons(value, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn to_string(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.to_string())
            },
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length {}", list.len());
    println!("linked list's content {}", list.to_string());
}
