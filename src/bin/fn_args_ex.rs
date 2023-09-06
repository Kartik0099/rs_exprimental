fn demo(v: usize, b: usize, c: impl Fn(usize, usize, usize) -> usize) -> usize {
    return c(v * b, v, b);
}

fn d2(x: usize, _y: usize, z: usize) -> usize {
    return x / z;
}

fn d4(x: usize, y: usize) -> bool {
    return x == y;
}

fn main() {
    let a: usize = (demo)(5, 6, |x: usize, _y: usize, z: usize| x / z);

    let b: usize = (demo)(5, 6, d2);

    let x = 5;

    let isX = (d4)(x, b);

    println!("x {} is y {} : {}", x, b, isX);

    fn d3(z: usize) -> bool {
        //can't capture dynamic environment in a fn item
        return x as usize == z;
    }

    println!("a: {} b: {}", a, b);
}
