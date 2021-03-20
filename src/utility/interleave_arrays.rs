use std::mem::swap;

pub fn interleave_arrays<T>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut arr: Vec<T> = vec![];
    let mut observed = a;
    let mut other = b;

    while !observed.is_empty() {
        arr.push(observed.remove(0));
        swap(&mut observed, &mut other);
    }

    let other_count = other.len();
    if other_count > 0 {
        arr.append(&mut other);
    }
    arr
}
