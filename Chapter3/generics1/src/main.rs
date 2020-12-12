fn make_tuple<T, S>(t: T, s: S) -> (T, S) {
    (t, s)
}

fn main() {
    let t1 = make_tuple(1, 2);
    let t2 = make_tuple("Hello", "World!");
    let t3 = make_tuple(vec![1, 2, 3], vec![4, 5]);
    let t4 = make_tuple(3, "years old");
}
