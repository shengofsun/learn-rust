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

fn call_me<F: Fn()>(f: F) {
    f();
}

fn input_functions() {
    let lambda = || {
        println!("This is a closure");
    };

    fn function() {
        println!("this is a function");
    }

    call_me(lambda);
    call_me(function);
}

fn create_fn() -> Box<Fn()> {
    let text = "fn".to_owned();
    Box::new(move || println!("this is {}", text))
}

fn create_fn_mut() -> Box<FnMut()> {
    let text = "fn_mut".to_owned();
    Box::new(move || println!("this is {}", text))
}

fn output_parameter() {
    let fn1 = create_fn();
    let mut fn2 = create_fn_mut();

    fn1();
    fn2();
}

fn std_iterator() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("2 in vec1 is {}", vec1.iter().any( |&x|x==2 ) );
    println!("2 in vec2 is {}", vec2.into_iter().any( |x|x==2 ));
}

fn main() {
    closure();
    closure_as_parameter();
    input_functions();
    output_parameter();

    std_iterator();
}

