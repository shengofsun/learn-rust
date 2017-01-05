type NanoSeconds = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type uint_64 = u64;

fn casting() {
    let nanoseconds: NanoSeconds = 32 as uint_64;
    let inch: Inch = 2 as uint_64;
    println!("{}, {}", nanoseconds, inch);
}

fn expression() {
    let x = 5u32;
    let y = {
        let x_square = x*x;
        let x_cube = x*x*x;

        x_cube + x_square + x
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
}

fn main() {
    casting();
    expression();
}
