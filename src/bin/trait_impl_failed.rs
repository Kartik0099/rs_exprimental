struct Circle {
    radius: f64,
    x: f64,
    y: f64,
}

// trait Shape {
//     fn area(&self) -> f64;
// }

// impl dyn Shape {
//     fn area(&self) -> f64 {
//         return 3.14 * self.radius * self.radius;
//     }
// }

trait Add {
    fn add(&self, other: &Self) -> Self;
}

// add `dyn` keyword before this trait: `dyn
// impl Add {}

//here number suggest how many try i dit
//the trait `Add` cannot be made into an object
//1. impl dyn Add {}

//2. impl dyn Add {
//     fn add(&self, other: &Self) -> Self {
//         return self + other;
//     }
// }

//3. impl dyn Add {
//     fn add(first: Box<dyn Add>, other: Box<dyn Add>) -> Box<dyn Add> {
//         return first + other;
//     }
// }

fn main() {}
