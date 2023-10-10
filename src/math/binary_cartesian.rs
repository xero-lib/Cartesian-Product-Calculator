pub fn cartesian_product<T: Clone, J: Clone>(a: &[T], b: &[J]) -> Vec<(T, J)> {
    let mut out: Vec<(T, J)> = vec![];
    for i in a {
        for j in b {
            out.push((i.clone(), j.clone()));
        }
    }
    out
}