fn main() {
    println!("{:?}", cartesian_product(&[1, 2, 3], &[4, 5, 6]));
}

// binary cartesian product
fn cartesian_product<T: Clone>(a: &[T], b: &[T]) -> Vec<(T, T)> {
    let mut out: Vec<(T, T)> = vec![];
    for i in a {
        for j in b {
            out.push((i.clone(), j.clone()));
        }
    }
    out
}