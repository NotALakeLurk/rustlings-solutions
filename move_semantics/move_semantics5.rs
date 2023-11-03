// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let mut x = 100;

    let y = &mut x;
    // Move this line up to group mutable accesses
    // Rust can apparently infer that `y` can be dropped after this,
    // Allowing the 'only one mutable reference' rule to be fufilled
    *y += 100;

    let z = &mut x;
    *z += 1000;

    assert_eq!(x, 1200);
}
