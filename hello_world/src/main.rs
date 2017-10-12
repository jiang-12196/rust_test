use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}",slice[0]);
    println!("first element of the slice: {}",slice[3]);
    println!("the slice has {} elements", slice.len());
}

fn slice_print() {
    let xs: [i32; 5] = [1,2,3,4,5];
    let ys: [i32; 500] = [0; 500];
    println!("first element of the slice: {}",xs[0]);
    println!("second element of the slice: {}",xs[1]);
    println!("array size : {}", xs.len());

    println!("array occupies {} bytes", mem::size_of_val(&xs));

    analyze_slice(&xs);

    analyze_slice(&ys);

    analyze_slice(&ys[1 .. 4]);

}


enum Person {
    Engineer,
    Scientist,
    Height(i32),
    Weight(i32),
    Info {
        name: String,
        height: i32
    }
}

fn inspect(p: Person) {
    match p {
        Person::Engineer => println!("is engineer"),
        Person::Scientist => println!("is scientist"),
        Person::Height(i) => println!("has a height of {}", i),
        Person::Weight(i) => println!("has a weight of {}", i),
        Person::Info {name, height} => {
            println!("{} is {} tall!", name, height);
        },
    }
}

fn enum_test() {
    let person = Person::Height(180);
    let amira = Person::Weight(160);
    let dave = Person::Info{ name: "Dave".to_owned(), height: 160 };
    let rohan = Person::Engineer;
    let rebecca = Person::Scientist;

    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rohan);
    inspect(rebecca);
}

use List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn list_test() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(2);
    list = list.prepend(4);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}


fn main() {
    println!("Hello, world!");
    // slice_print();
    enum_test();
    list_test();

}
