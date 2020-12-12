#[derive(Eq, PartialEq)]
struct A(i32);

#[derive(PartialEq, PartialOrd)]
struct B(f32);

#[derive(Copy, Clone)]
struct C;

#[derive(Clone)]
struct D;

#[derive(Debug)]
struct E;

#[derive(Default)]
struct F;

fn main() {
    println!("{:?}", A(0) == A(1));
    println!("{:?}", B(1.0) > B(0.0));
    
    let c0 = C;
    let _c1 = c0;
    let _c2 = c0;

    let d0 = D;
    let _d1 = d0.clone();

    println!("{:?}", E);

    let _f = F::default();
}
