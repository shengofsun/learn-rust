fn closure() {
    let closure_annotated = |i: i32| -> i32 { i+1 };
    let closure_inferred = |i: i32| i+1;

    let i = 1;
    println!("annotated closure: {}", closure_annotated(i));
    println!("inferred closure: {}", closure_inferred(i));

    let one = || 1;
    println!("closure return one: {}", one());
}

fn apply<F>(function: F) where F: FnOnce() {
    function();
}

fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {
    f(3)
}

fn closure_as_parameter() {
    // use std::mem;

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();
    let diary = || {
        // works fine by reference
        println!("say {:?}", greeting);
        // only works fine by mutable reference
        farewell.push_str("!!!");
        // only works by value
        // mem::drop(farewell);
        println!("{}", farewell);
    };

    apply(diary);
    let double = |i: i32| i*2;
    println!("double 3: {}", apply_to_3(double));
}

fn main() {
    closure();
    closure_as_parameter();
}
