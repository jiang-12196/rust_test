fn lambda() {
    fn function(i: i32) -> i32 { i + 1 }

    let closure_annotated = |i: i32| -> i32 { i+ 1};
    let closure_inferred = | i| i + 1;
    let i = 1;
    println!("function : {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}",closure_inferred(i));
    let one = || 1;
    println!("closure returning one: {}", one());
}


fn mem_test() {
    use std::mem;

    let color = "green";

    let print = || println!("color: {}", color);

    print();
    print();

    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("count: {}", count);
    };

    inc();
    inc();

    // let reborrow = &mut count;
    let movable = Box::new(3);

    let consume = || {
        println!("movable: {:?}",movable);
        mem::drop(movable);
    };

    consume()
}

fn main() {
    lambda();
    mem_test();
}