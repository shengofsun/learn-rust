fn if_else_control() {
    let n = 5;

    if n < 0 {
        println!("n is negtive");
    } else if n > 0 {
        println!("n is positive");
    } else {
        println!("n is zero");
    };

    let big_n =
        if n<10 && n>-10 {
            println!("n is small, incresed ten fold");
            n*10
        } else {
            println!("n is big");
            n/2
        };

    println!("{} -> {}", n, big_n);
}

fn loop_control() {
    let mut n = 0u32;

    loop {
        n+=1;
        if n==3 {
            println!("three");
            continue;
        }
        println!("{}", n);
        if n == 5 {
            println!("finished");
            break;
        }
    }
}

#[allow(unreachable_code)]
fn nested_loop() {
    println!("enter nested loop");

    'outer: loop {
        println!("enter the outer loop");
        'inner: loop {
            println!("enter the inner loop");
            break 'outer;
        }
        println!("never executed");
    }

    println!("exit the loop");
}

fn while_control() {
    let mut a = 1;
    let mut b = 2;
    let mut c = 0;
    let mut count = 10_i32;

    while count > 0 {
        c = a+b;
        a = b;
        b = c;
        count-=1;
    }

    println!("result is {}", c);
}

fn for_loop() {
    for n in 1..20 {
        if n%15 == 0 {
            println!("fuzzbuzz");
        }
        else if n%3 == 0 {
            println!("fuzz");
        }
        else if n%5==0 {
            println!("buzz");
        }
        else {
            println!("{}", n);
        }
    }
}

fn pattern_match() {
    let number = 13;
    println!("tell me abount {}", number);
    match number {
        1 => println!("one"),
        2 | 3 | 5 | 7 | 11 => println!("a prime"),
        13...19 => println!("a teen"),
        _ => println!("not special"),
    };

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} => {}", boolean, binary);

    let pair = (0, 5);
    println!("tell me abount {:?}", pair);
    match pair {
        (0, y) => println!("first is zero, and y is {}", y),
        (x, 0) => println!("x is {}, and y is zero", x),
        _ => println!("it doesn't matter"),
    }

    let reference = &4;
    match reference {
        &val => println!("got a value by destructure {:?}", val),
    }

    match *reference {
        val => println!("got a value by dereference {:?}", val),
    }

    let ref is_a_reference = 4;

    match is_a_reference {
        ref val => println!("got a value by destructure {:?}", val),
    }

    match *is_a_reference {
        val => println!("got a value by deference {:?}", val),
    }

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("got a value by reference to value {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("we add 10, {:?}", m);
        },
    }

    match mut_value {
        mut m => {
            m+=10;
            println!("add 10 again, {:?}", m);
        }
    }

    println!("mut value: {:?}", mut_value);
}

fn destructure() {
    #[derive(Debug)]
    struct Foo { x:(u32, u32), y: u32 }

    let foo = Foo { x: (5, 6), y: 7 };
    let Foo { x: (a, b), y: c } = foo;

    println!("a: {}, b: {}, c: {}", a, b, c);
    println!("foo: {:?}", foo);
}

fn match_guard(pair: (i32, i32)) {
    match pair {
        (x, y) if x==y => println!("these are twins, x={}", x),
        (x, y) if x+y==0 => println!("these are opposite, x: {}, y: {}", x, y),
        _ => println!("not important"),
    }
}

fn binding() {
    fn age() -> u32 { 15 }

    match age() {
        0 => println!("zero"),
        n @ 1...12 => println!("A child of age {:?}", n),
        n @ 13...19 => println!("A teen of age {:?}", n),
        _ => println!("very big"),
    }

    let number = Some(7);
    let letter: Option<i32> = None;

    if let Some(i)=number {
        println!("destructure {:?} from {:?}", i, number);
    }

    if let Some(_)=letter {
        println!("destructured ok");
    } else {
        println!("destructured failed.")
    }

    let mut optional = Some(0);
    while let Some(i)=optional {
        if i>9 {
            println!("greater than 9, quit");
            optional = None;
        } else {
            println!("i is {}, continue", i);
            optional = Some(i+1);
        }
    }
}

fn main() {
    println!("Hello, world!");

    if_else_control();
    loop_control();
    nested_loop();
    while_control();
    for_loop();

    pattern_match();
    destructure();
    match_guard((1, 2));
    match_guard((1, -1));
    match_guard((2, 2));
    binding();
}
