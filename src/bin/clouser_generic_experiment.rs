//cannot define inherent `impl` for foreign type
// fn add<T: std::ops::Add<Output = T>>(a: T, b: F) -> T  where F:impl Fn(T,T)->T{
//     return a + b;
// }

//use parentheses to call this type parameter: `(/* T */, /* T */)`
// fn add<T: std::ops::Add<Output = T>>(a: T, b: impl Fn(T,T)->T) -> T  {
//     return a + b;
// }

//a type parameter with a similar name exists: `T`
// fn add<T: std::ops::Add<Output = T>>(a: T, b: impl Fn(U,U)->U) -> T where U:std::ops::Add<Output = U> {
//     return a + b;
// }

/*
some one comment
Rust doesn't support generics for closures. The thing you can do instead is create your own trait with a generic method of the desired signature (plus a &selfargument if you want to support the equivalent of variable capture in Fn(…)), and then pass an instance of that trait instead of an instance of some Fn trait. You'll lose the benefit of closure syntax and automatic variable captures, but in principle, it's possible to express everything that one would need to express.
https://users.rust-lang.org/t/use-generics-in-closure/86679
 */

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    return a + b;
}

fn main() {
    let x = 5;

    //cannot find type `T` in this scope
    // Syntax Error: expected PIPEr
    // let z:usize = |num:T|->T{
    //     return num;
    // }

    //`for<...>` binders for closures are experimental
    // let a = for<T> |s: &str, t: T| {
    //     return s.len() + t;
    // }

    let y = 6;
    println!("add {} + {} = {}", x, y, add(x, y));
}
