// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    // let mut new_str = String::new();
    // for char in input.chars(){
    //     if char != ' ' && != ' '{
    //         new_str.push(char);
    //     }
    // }
    let new_str = String::from(input.trim());
    // let mut new_str = String::from(input);
    // new_str.remove_matches(" ");
    // new_str = String::from(format!(""))

    return new_str
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let new_str = String::from(input);
    return new_str + " world!"
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    // let mut new_str = String::new();
    // let mut tmp_str = String::new();
    // for char in input.chars(){
    //     tmp_str.push(char);
    //     if char == ' ' || char == '\n' {
    //         if tmp_str == "cars "{
    //             new_str.push_str("balloons ")
    //         }
    //         else{
    //             new_str.push_str(&tmp_str);
    //         }
    //         tmp_str.clear();
    //     }
    // }
    let new_str = input.replace("cars", "balloons");
    return new_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
