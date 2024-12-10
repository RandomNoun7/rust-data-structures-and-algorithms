pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.curr >= self.max {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        return Some(res);
    }
}

fn main() {
    let mut st = Stepper {
        curr: 2,
        step: 3,
        max: 15,
    };

    loop {
        match st.next() {
            Some(v) => println!("loop {}", v),
            None => break,
        }
    }

    let mut st2 = Stepper {
        curr: 3,
        step: 4,
        max: 15,
    };

    while let Some(n) = st2.next() {
        println!("while, {}", n);
    }

    let it = Stepper {
        curr: 5,
        step: 10,
        max: 50,
    };

    for i in it {
        println!("for loop {}", i);
    }

    let mut n = 0;
    while n < 10 {
        n += 1;
        println!("Hello, {}!", n);
    }

    for i in 1..10 {
        println!("for loop {}", i);
    }

    println!("All done");
}
