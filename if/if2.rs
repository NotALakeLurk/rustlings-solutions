// if2.rs
//
// Step 1: Make me compile!
// Step 2: Get the bar_for_fuzz and default_to_baz tests passing!
//
// Execute `rustlings hint if2` or use the `hint` watch subcommand for a hint.

pub fn foo_if_fizz(fizzish: &str) -> &str {
    // Ok, I know this section is about if, but wouldn't match just feel so much better here?
    // Like: `match fizzish { "fizz" => "foo", "fuzz" => "bar", _ => "baz" }`
    // Or, honestly, there's probably some other, better way to do this too lol. Dicts?

    // Anyway... Just add a check for "fuzz" -> "bar", and default to "baz"
    if fizzish == "fizz" {
        "foo"
    } else if fizzish == "fuzz" {
        "bar"
    } else {
        "baz"
    }
}

// No test changes needed!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        assert_eq!(foo_if_fizz("fizz"), "foo")
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar")
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz")
    }
}
