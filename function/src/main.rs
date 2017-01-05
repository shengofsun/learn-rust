use std::fmt;

fn fizzbuzz(number: i32) {
    if is_divided(number, 15) {
        println!("fizzbuzz");
    } else if is_divided(number, 3) {
        println!("fizz");
    } else if is_divided(number, 5) {
        println!("buzz");
    } else {
        println!("not important");
    }
}

fn is_divided(d1: i32, d2: i32) -> bool {
    if d2==0 {
        return false;
    }
    d1%d2 == 0
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(a: f64, b: f64) -> Point {
        Point{x: a, y: b}
    }
}

#[derive(Debug)]
struct Rect {
    p1: Point,
    p2: Point,
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "p1: ({}, {})", self.p1.x, self.p1.y));
        try!(write!(f, ","));
        write!(f, "p2: ({}, {})", self.p2.x, self.p2.y)
    }
}

impl Rect {
    fn area(&self) -> f64 {
        let Point{ x: x1, y: y1 } = self.p1;
        let Point{ x: x2, y: y2 } = self.p2;

        ( (x1-x2)*(y1-y2) ).abs()
    }

    fn perimeter(&self) -> f64 {
        2.0*( (self.p1.x - self.p2.x).abs() + (self.p1.y - self.p2.y).abs() )
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);
impl Pair {
    fn destroy(self) {
        let Pair(a, b) = self;
        println!("destroyed pair({}, {})", a, b);
    }
}

fn methods() {
    let rect = Rect {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("the area is {}, the perimeter is {}", rect.area(), rect.perimeter());

    let mut rect = Rect {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    rect.translate(2.0, 2.0);
    println!("the translated rect is {}", rect);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
}

fn main() {
    for i in 1..10 {
        fizzbuzz(i);
    }

    methods();
}
