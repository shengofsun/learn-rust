static LANGUAGE: &'static str = "rust";
const THRESHOD: i32 = 16;

fn is_big(number: i32) -> bool {
    number > THRESHOD
}

fn main() {
    let n = 16;
    println!("the language is {}", LANGUAGE);
    println!("the threshold is {}", THRESHOD);

    println!("{} is {}", n, if is_big(n) { "big"} else { "small"} );
}
