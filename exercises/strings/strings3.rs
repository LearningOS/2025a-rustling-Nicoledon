// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// use std::mem::replace;

use std::arch::aarch64::int8x8_t;
use std::os::unix::raw::off_t;

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let mut str = String::new();
    for ch in input.chars() {
         if ch != ' ' {
             str.push(ch);
         }
    }
    return str;
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
       let str =  String::from(input);
       let world = " world";
       return str + world;
}

fn parse(input: &str) ->Vec<String> {
    let mut v = vec![];
    let line = String::from("");
    for ch in input.chars() {
        if ch != ' ' {
            line += ch;
        }else{
             v.push(line);
        }
    }
    return v;
}
fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let words= parse(input);
    let line = String::from("");
    for word in words {
        if word != "cars" {
            line += word;
        }else {
             line += "balloons";
        }
    }
    return line ;
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
