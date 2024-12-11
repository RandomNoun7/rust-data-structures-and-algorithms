#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn main() {
    let mut x = 34;
    let y = x;

    x += 5;
    println!("y = {}, x = {}", y, x);

    let mut p = Person {
        name: "Matt".to_string(),
        age: 35,
    };

    let p2 = p.clone();
    p.name.push_str(" Stoodley");

    println!("p = {:?}, p2 = {:?}", p, p2);

    let mut pnt = Point::new(3, 4);
    let pn2 = pnt;

    pnt.x += 3;

    println!("pnt = {:?}, p = {:?}", pnt, pn2)
}
