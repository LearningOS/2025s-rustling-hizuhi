// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.


#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        let x = 1;
        let y = 1;
        assert_eq!(x, y, "we are testing {} and {}", x, y);
    }
}
