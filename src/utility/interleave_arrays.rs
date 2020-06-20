use std::mem::swap;

pub fn interleave_arrays<T>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut arr: Vec<T> = vec![];
    let mut observed = a;
    let mut other = b;
    let mut count = observed.len();

    while count > 0 {
        arr.push(observed.swap_remove(0));
        swap(&mut observed, &mut other);
        count = observed.len();
    }

    let other_count = other.len();
    if other_count > 0 {
        arr.append(&mut other);
    }
    arr
}