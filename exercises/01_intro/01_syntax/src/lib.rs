// TODO: fix the function signature below to make the tests pass.
//  Make sure to read the compiler error message—the Rust compiler is your pair programming
//  partner in this course and it'll often guide you in the right direction!
//
// The input parameters should have the same type of the return type.
fn compute(a: u32, b: u32) -> u32 {      /* I'll differentiate my comments using /**/ We need to add data types to the function arguments*/
    // Don't touch the function body.
    a + b * 2
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 5);
    }
}