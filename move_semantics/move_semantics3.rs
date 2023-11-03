// move_semantics3.rs
//
// Make me compile without adding new lines -- just changing existing lines! (no
// lines with multiple semicolons necessary!)
//
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    // Clone first vec to pass into function that takes owership
    // Note that we don't specify that the clone is mutable
    let vec1 = fill_vec(vec0.clone());

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

// Change to take a mutable Vec
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);

    vec
}
