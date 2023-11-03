// quiz1.rs
//
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!
// Write a function that calculates the price of an order of apples given the
// quantity bought.
//
// No hints this time ;)

// Put your function here!

// Maybe this should use `u32`s instead, but there's little reason not to 
// And doing it would be inconvinient (also the tests wouldn't like it)
fn calculate_price_of_apples(quantity: i32) -> i32 {
    // Once again, I'd love to use a match expression here, but this is an if quiz

    // Set the price based on the quantity:
    // x > 40 => price = 1, 
    // x <= 40 => price = 2,
    let single_price = if quantity > 40 { 1 } else { 2 };

    // Calculate and return total price
    quantity * single_price
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
