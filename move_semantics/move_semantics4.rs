// move_semantics4.rs
//
// Refactor this code so that instead of passing `vec0` into the `fill_vec`
// function, the Vector gets created in the function itself and passed back to
// the main function.
//
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    // Okay, apparently we can just move this line into the function,
    // Not sure how I'd do this any other way lol
    // let vec0 = vec![22, 44, 66];

    let mut vec1 = fill_vec();

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument - don't change this!
fn fill_vec() -> Vec<i32> {
    // Move the original vec creation into this funcion I guess
    // This feels weird because I want to keep it in main, but there's literally no other way to do
    // this
    let mut vec = vec![22, 44, 66];

    vec.push(88);

    vec
}
